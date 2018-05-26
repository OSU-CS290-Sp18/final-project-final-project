use std::io::Cursor;
use std::marker::PhantomData;
use std::sync::Arc;

use api::*;
use error::Error;
use resources::*;

use futures::{future, Async, Future, Poll, Stream};
use hyper::{self, Request, Response};
use hyper::client::HttpConnector;
use hyper::body::Body;
use serde::de::DeserializeOwned;
use serde_json;
use url::form_urlencoded::Serializer;

pub enum QueryState {
    AwaitingRedirect {
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
    http_client: Arc<hyper::Client<HttpConnector, Body>>,
}

impl Client {
    pub fn new() -> Client {
        Client {
            http_client: Arc::new(hyper::Client::new()),
        }
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
