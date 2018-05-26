use std::io::Cursor;
use std::marker::PhantomData;

use error::Error;

use futures::{future, Async, Future, Poll, Stream};
use hyper::{self, Request};
use hyper::client::HttpConnector;
use hyper::body::Body;
use serde::de::DeserializeOwned;
use serde_json;

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
}
