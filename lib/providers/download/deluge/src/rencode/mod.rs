mod constants;
mod decoder;
mod encoder;
pub mod error;
pub mod objects;

pub use self::error::Error;
pub use self::objects::{Float, Int, Object};

use self::decoder::Decoder;
use self::encoder::Encoder;

pub fn encode(t: Vec<Object>) -> Result<Vec<u8>, Error> {
    let encoder = Encoder::new();
    // TODO: This is honestly super hard to read and really stupid; really need to find a better
    // way to encode.
    encoder.into_bytes(Object::List(vec![Object::List(t)]))
}

pub fn decode(data: Vec<u8>) -> Option<Object> {
    let mut decoder = Decoder::new(data);
    decoder.decode_next()
}
