use std::io::Cursor;
use std::marker::PhantomData;
use std::sync::Arc;

use api::*;
use error::Error;
use resources::*;

use futures::{Async, Future, Poll, Stream};
use http::uri::{Parts, Scheme};
use hyper::{self, Request, Response, Uri};
use hyper::client::HttpConnector;
use hyper::body::Body;
#[cfg(feature = "tls")]
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde_json;
use url::form_urlencoded::Serializer;

pub enum QueryState {
    AwaitingRedirect {
        #[cfg(feature = "tls")]
        http_client: Arc<hyper::Client<HttpsConnector<HttpConnector>, Body>>,
        #[cfg(not(feature = "tls"))]
        http_client: Arc<hyper::Client<HttpConnector, Body>>,
        inner: Box<Future<Item=Response<Body>, Error=hyper::Error>>,
    },
    AwaitingData(Box<Future<Item=Vec<u8>, Error=Error>>),
}

pub struct QueryFuture<R> {
    state: QueryState,
    _marker: PhantomData<R>,
}

unsafe impl<R: DeserializeOwned> Send for QueryFuture<R> {}

impl<R: DeserializeOwned> Future for QueryFuture<R> {
    type Item = R;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let state = match self.state {
                QueryState::AwaitingRedirect { ref http_client, ref mut inner } => {
                    let res = try_ready!(inner.poll());
                    match res.status().as_u16() {
                        301 | 302 => {
                            let url = res.headers().get("Location")
                                .map(|loc| loc.to_str().map_err(Error::from))
                                .ok_or(Error::MissingLocationHeader);

                            match url {
                                Ok(Ok(url)) => {
                                    #[cfg(feature = "tls")]
                                    let url = url_to_https(url)?;
                                    let req = Request::get(url).body(Body::empty())?;
                                    let inner = http_client.request(req)
                                        .and_then(|res| res.into_body().concat2())
                                        .map(|chunk| chunk.to_vec())
                                        .map_err(Error::from);

                                    QueryState::AwaitingData(Box::new(inner))
                                }
                                Ok(Err(e)) => { return Err(e); }
                                Err(e) => { return Err(e); }
                            }
                        }
                        404 => { return Err(Error::ShowNotFound); },
                        _ => { return Err(Error::UnexpectedResponse); },
                    }
                }

                QueryState::AwaitingData(ref mut fut) => {
                    let bytes = try_ready!(fut.poll());
                    let cursor = Cursor::new(bytes);
                    let deserialized: R = serde_json::from_reader(cursor)?;
                    return Ok(Async::Ready(deserialized));
                }
            };

            self.state = state;
        }
    }
}

pub struct Client {
    #[cfg(feature = "tls")]
    http_client: Arc<hyper::Client<HttpsConnector<HttpConnector>, Body>>,
    #[cfg(not(feature = "tls"))]
    http_client: Arc<hyper::Client<HttpConnector, Body>>,
}

impl Client {
    #[cfg(not(feature = "tls"))]
    pub fn new() -> Client {
        Client {
            http_client: Arc::new(hyper::Client::new()),
        }
    }

    #[cfg(feature = "tls")]
    pub fn new() -> Result<Client, Error> {
        let mut connector = HttpsConnector::new(4)?;
        connector.force_https(true);

        Ok(Client {
            http_client: Arc::new(hyper::Client::builder().build(connector))
        })
    }

    fn create_future<R: DeserializeOwned>(&self, req: Request<Body>) -> QueryFuture<R> {
        let inner = self.http_client.request(req)
            .and_then(|res| res.into_body().concat2())
            .map(|chunk| chunk.to_vec())
            .map_err(Error::from);

        QueryFuture {
            state: QueryState::AwaitingData(Box::new(inner)),
            _marker: PhantomData,
        }
    }

    pub fn search_shows(&self, query: &str)
        -> Result<QueryFuture<Vec<ShowSearchResult>>, Error>
    {
        let req = create_request("/search/shows", Some(vec![("q", query)]), None)?;
        Ok(self.create_future(req))
    }

    pub fn search_show(&self, query: &str, embed: Option<LinkEmbed>)
        -> Result<QueryFuture<TVShow>, Error>
    {
        let req = create_request(
            "/singlesearch/shows",
            Some(vec![("q", query)]),
            embed
        )?;
        Ok(self.create_future(req))
    }

    pub fn lookup_show(&self, show: ShowLookup)
        -> Result<QueryFuture<Option<TVShow>>, Error>
    {
        // This request should return either a redirect or a 404, so the future
        // resolves to a URL, the content of the location header.
        let initial_lookup = create_request(
            "/lookup/shows",
            Some(vec![show.as_query_param()]),
            None
        )?;
        let inner = Box::new(self.http_client.request(initial_lookup));
        let http_client = self.http_client.clone();

        Ok(QueryFuture {
            state: QueryState::AwaitingRedirect { http_client, inner },
            _marker: PhantomData,
        })
    }

    pub fn search_people(&self, query: &str)
        -> Result<QueryFuture<Vec<PeopleSearchResult>>, Error>
    {
        let req = create_request("/search/people", Some(vec![("q", query)]), None)?;
        Ok(self.create_future(req))
    }

    pub fn today_schedule(&self, country: Option<&str>, date: Option<&str>)
        -> Result<QueryFuture<Vec<Episode>>, Error>
    {
        let params = vec!["country", "date"].into_iter()
            .zip(vec![country, date])
            .filter(|(_, p)| p.is_some())
            .map(|(k, p)| (k, p.unwrap()))
            .collect::<Vec<(&str, &str)>>();
        let params = if params.is_empty() { None } else { Some(params) };

        let req = create_request("/schedule", params, None)?;
        Ok(self.create_future(req))
    }

    pub fn full_schedule(&self) -> Result<QueryFuture<Vec<Episode>>, Error> {
        let req = create_request("/schedule/full", None, None)?;
        Ok(self.create_future(req))
    }

    pub fn show_main_info(&self, id: &str) -> Result<QueryFuture<TVShow>, Error> {
        let req = create_request(&["/shows/", id].join(""), None, None)?;
        Ok(self.create_future(req))
    }

    pub fn show_episode_list(&self, id: &str, specials: bool)
        -> Result<QueryFuture<Vec<Episode>>, Error>
    {
        let path = &["/shows/", id, "/episodes"].join("");
        // I'm not sure how specials actually work. I've tried looking at the JSON
        // returned when the specials param is set, but I've been unable to find any
        // shows that return any special episodes.
        let params = if specials { Some(vec![("specials", "1")]) }  else { None };
        let req = create_request(path, params, None)?;
        Ok(self.create_future(req))
    }

    pub fn show_episode_by_number(&self, id: &str, season: u64, episode: u64)
        -> Result<QueryFuture<Episode>, Error>
    {
        let path = &["/shows/", id, "/episodebynumber"].join("");
        let season = season.to_string();
        let episode = episode.to_string();
        let params = Some(vec![("season", season.as_str()), ("number", episode.as_str())]);
        let req = create_request(path, params, None)?;
        Ok(self.create_future(req))
    }

    pub fn show_episodes_by_date(&self, id: &str, date: &str)
        -> Result<QueryFuture<Vec<Episode>>, Error>
    {
        let path = &["/shows/", id, "/episodesbydate"].join("");
        let req = create_request(path, Some(vec![("date", date)]), None)?;
        Ok(self.create_future(req))
    }

    pub fn show_seasons(&self, id: &str)
        -> Result<QueryFuture<Vec<TVShowSeason>>, Error>
    {
        let path = &["/shows/", id, "/seasons"].join("");
        let req = create_request(path, None, None)?;
        Ok(self.create_future(req))
    }

    pub fn season_episodes(&self, season_id: &str)
        -> Result<QueryFuture<Vec<Episode>>, Error>
    {
        let path = &["/seasons/", season_id, "/episodes"].join("");
        let req = create_request(path, None, None)?;
        Ok(self.create_future(req))
    }

    pub fn show_cast(&self, id: &str)
        -> Result<QueryFuture<Vec<CastPerson>>, Error>
    {
        let path = &["/shows/", id, "/cast"].join("");
        let req = create_request(path, None, None)?;
        Ok(self.create_future(req))
    }

    pub fn show_crew(&self, id: &str)
        -> Result<QueryFuture<Vec<CrewPerson>>, Error>
    {
        let path = &["/shows/", id, "/crew"].join("");
        let req = create_request(path, None, None)?;
        Ok(self.create_future(req))
    }

    pub fn show_akas(&self, id: &str)
        -> Result<QueryFuture<Vec<AKA>>, Error>
    {
        let path = &["/shows/", id, "/akas"].join("");
        let req = create_request(path, None, None)?;
        Ok(self.create_future(req))
    }

    pub fn show_index(&self, page: Option<u64>)
        -> Result<QueryFuture<Vec<TVShow>>, Error>
    {
        let page = page.unwrap_or(1).to_string();
        let req = create_request("/shows", Some(vec![("page", &page)]), None)?;
        Ok(self.create_future(req))
    }

    pub fn person_main_info(&self, person_id: &str)
        -> Result<QueryFuture<Person>, Error>
    {
        let req = create_request(&["/people/", person_id].join(""), None, None)?;
        Ok(self.create_future(req))
    }
}

fn create_request(
    path: &str,
    query_params: Option<Vec<(&str, &str)>>,
    embed: Option<LinkEmbed>)
    -> Result<Request<Body>, Error>
{
    let url = if query_params.is_some() || embed.is_some() {
        let mut serializer = Serializer::new(String::new());

        if let Some(query_params) = query_params {
            for (key, val) in query_params {
                serializer.append_pair(key, val);
            }
        }

        if let Some(link_embed) = embed {
            match link_embed {
                LinkEmbed::Single(embed) => {
                    serializer.append_pair("embed", embed);
                }
                LinkEmbed::Multiple(embeds) => {
                    for embed in embeds {
                        serializer.append_pair("embed[]", embed);
                    }
                }
            }
        }

        [TVMAZE_BASE_URL, path, "?", &serializer.finish()].join("")
    } else {
        [TVMAZE_BASE_URL, path].join("")
    };

    Request::get(url).body(Body::empty()).map_err(Error::from)
}

#[cfg(feature = "tls")]
fn url_to_https(url: &str) -> Result<Uri, Error> {
    let uri: Uri = url.parse()?;
    let mut parts: Parts = uri.into();
    parts.scheme = Some(Scheme::HTTPS);
    Uri::from_parts(parts).map_err(Error::from)
}
