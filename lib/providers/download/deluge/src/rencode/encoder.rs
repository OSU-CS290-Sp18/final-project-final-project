use std::collections::HashMap;

use super::constants::*;
use super::Error;
use super::{Float, Int, Object};

use bincode::config;

pub struct Encoder {
    data: Vec<u8>,
}

impl Encoder {
    pub fn new() -> Encoder {
        Encoder { data: Vec::new() }
    }

    pub fn into_bytes(mut self, obj: Object) -> Result<Vec<u8>, Error> {
        self.encode_object(obj).map(|_| self.data)
    }

    pub fn encode_object(&mut self, obj: Object) -> Result<(), Error> {
        match obj {
            Object::Map(m) => self.encode_map(m),
            Object::List(l) => self.encode_list(l),
            Object::Bool(b) => self.encode_bool(b),
            Object::Float(f) => self.encode_float(f),
            Object::Int(i) => self.encode_int(i),
            Object::Str(s) => self.encode_string(s),
            Object::Bytes(b) => self.encode_string_bytes(b),
        }
    }

    pub fn encode_map(&mut self, map: HashMap<String, Object>) -> Result<(), Error> {
        if map.len() < (DICT_FIXED_COUNT as usize) {
            self.data.push(DICT_FIXED_START + map.len() as u8);

            for (k, v) in map {
                self.encode_string(k)?;
                self.encode_object(v)?;
            }
        } else {
            self.data.push(CHR_DICT);

            for (k, v) in map {
                self.encode_string(k)?;
                self.encode_object(v)?;
            }

            self.data.push(CHR_TERM);
        }

        Ok(())
    }

    pub fn encode_list(&mut self, list: Vec<Object>) -> Result<(), Error> {
        if list.len() < (LIST_FIXED_COUNT as usize) {
            self.data.push(LIST_FIXED_START + list.len() as u8);

            for o in list {
                self.encode_object(o)?;
            }
        } else {
            self.data.push(CHR_LIST);

            for o in list {
                self.encode_object(o)?;
            }

            self.data.push(CHR_TERM);
        }

        Ok(())
    }

    pub fn encode_bool(&mut self, b: bool) -> Result<(), Error> {
        let c = if b { CHR_TRUE } else { CHR_FALSE };
        self.data.push(c);
        Ok(())
    }

    pub fn encode_float(&mut self, f: Float) -> Result<(), Error> {
        match f {
            Float::F32(f) => {
                self.data.append(&mut config().big_endian().serialize(&f)?);
            }
            Float::F64(f) => {
                self.data.append(&mut config().big_endian().serialize(&f)?);
            }
        }

        Ok(())
    }

    pub fn encode_int(&mut self, i: Int) -> Result<(), Error> {
        match i {
            Int::I8(i) => {
                self.data.append(&mut config().big_endian().serialize(&i)?);
            }
            Int::I16(i) => {
                self.data.append(&mut config().big_endian().serialize(&i)?);
            }
            Int::I32(i) => {
                self.data.append(&mut config().big_endian().serialize(&i)?);
            }
            Int::I64(i) => {
                self.data.append(&mut config().big_endian().serialize(&i)?);
            }
        }

        Ok(())
    }

    pub fn encode_string(&mut self, s: String) -> Result<(), Error> {
        self.encode_string_bytes(s.into_bytes())
    }

    pub fn encode_string_bytes(&mut self, b: Vec<u8>) -> Result<(), Error> {
        if b.len() < (STR_FIXED_COUNT as usize) {
            self.data.push(STR_FIXED_START + b.len() as u8);
        } else {
            let len = b.len().to_string();
            self.data.extend_from_slice(len.as_bytes());
            self.data.push(58);
        }

        self.data.extend(b);

        Ok(())
    }
}
