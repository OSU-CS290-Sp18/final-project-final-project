#[macro_use]
extern crate futures;
extern crate http;
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;
extern crate url;

mod api;
mod client;
mod error;
mod resources;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
