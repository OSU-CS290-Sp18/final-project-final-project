use std::io::Cursor;
use std::marker::PhantomData;

use api::*;
use error::Error;
use resources::TVShow;

use futures::{future, Async, Future, Poll, Stream};
use hyper::{self, Request};
use hyper::client::HttpConnector;
use hyper::body::Body;
use serde::de::DeserializeOwned;
use serde_json;
use url::form_urlencoded::Serializer;

pub struct QueryFuture<R> {
    inner: Box<Future<Item=Vec<u8>, Error=hyper::Error>>,
    _marker: PhantomData<R>,
}

unsafe impl<R: DeserializeOwned> Send for QueryFuture<R> {}

impl<R: DeserializeOwned> Future for QueryFuture<R> {
    type Item = R;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let bytes = try_ready!(self.inner.poll());
        let cursor = Cursor::new(bytes);
        let deserialized: R = serde_json::from_reader(cursor)?;
        Ok(Async::Ready(deserialized))
    }
}

pub struct Client {
    http_client: hyper::Client<HttpConnector, Body>,
}

impl Client {
    pub fn new() -> Client {
        Client {
            http_client: hyper::Client::new(),
        }
    }

    fn create_future<R: DeserializeOwned>(&self, req: Request<Body>) -> QueryFuture<R> {
        let inner = self.http_client.request(req)
            .and_then(|res| res.into_body().concat2())
            .map(|chunk| chunk.to_vec());

        QueryFuture {
            inner: Box::new(inner),
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
    let mut url = String::from(TVMAZE_BASE_URL) + path;

    if query_params.is_some() || embed.is_some() {
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

        url += "?";
        url += &serializer.finish();
    }

    Request::get(url).body(Body::empty()).map_err(Error::from)
}
