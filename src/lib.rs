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

pub mod api;
pub mod client;
pub mod error;
pub mod resources;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
