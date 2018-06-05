use std::collections::{HashMap, VecDeque};

use super::constants::*;
use super::{Float, Int, Object};

use bincode::config;

pub struct Decoder {
    data: VecDeque<u8>,
}

impl Decoder {
    pub fn new(data: Vec<u8>) -> Decoder {
        Decoder { data: data.into() }
    }

    pub fn decode_next(&mut self) -> Option<Object> {
        let token = self.data.front()?.clone();

        match token {
            t if is_map(t) || is_fixed_map(t) => self.decode_map(),
            t if is_list(t) || is_fixed_list(t) => self.decode_list(),
            t if is_num(t) => self.decode_num(),
            t if is_bool(t) => self.decode_bool(),
            t if is_float(t) => self.decode_float(),
            t if is_any_int(t) => self.decode_int(),
            _ => self.decode_bytes(),
        }
    }

    fn decode_map(&mut self) -> Option<Object> {
        let token = self.data.pop_front()?;
        let mut map = HashMap::new();

        if is_fixed_map(token) {
            for _ in 0..(token - DICT_FIXED_START) {
                let key = match self.decode_bytes()? {
                    Object::Str(s) => s,
                    _ => {
                        return None;
                    }
                };
                let value = self.decode_next()?;
                map.insert(key, value);
            }
        } else {
            while *self.data.front()? != CHR_TERM {
                let key = match self.decode_bytes()? {
                    Object::Str(s) => s,
                    _ => {
                        return None;
                    }
                };
                let value = self.decode_next()?;
                map.insert(key, value);
            }

            self.data.pop_front()?;
        }

        Some(Object::Map(map))
    }

    fn decode_list(&mut self) -> Option<Object> {
        let token = self.data.pop_front()?;
        let mut list = Vec::new();

        if is_fixed_list(token) {
            for _ in 0..(token - LIST_FIXED_START) {
                list.push(self.decode_next()?);
            }
        } else {
            while *self.data.front()? != CHR_TERM {
                list.push(self.decode_next()?);
            }

            self.data.pop_front()?;
        }

        Some(Object::List(list))
    }

    fn decode_num(&mut self) -> Option<Object> {
        let _token = self.data.pop_front()?;
        let mut bytes = Vec::new();

        while *self.data.front()? != CHR_TERM {
            bytes.push(self.data.pop_front()?);
        }

        let num = String::from_utf8(bytes).ok()?;
        if num.contains('.') {
            let float: f64 = num.parse::<f64>().ok()?;
            Some(Object::Float(Float::F64(float)))
        } else {
            // To be honest, I have no idea how large this number could get...
            let int: i64 = num.parse::<i64>().ok()?;
            Some(Object::Int(Int::I64(int)))
        }
    }

    fn decode_bool(&mut self) -> Option<Object> {
        let token = self.data.pop_front()?;

        if token == CHR_TRUE {
            Some(Object::Bool(true))
        } else {
            Some(Object::Bool(false))
        }
    }

    fn decode_float(&mut self) -> Option<Object> {
        let token = self.data.pop_front()?;
        let len = match token {
            CHR_FLOAT32 => 4,
            CHR_FLOAT64 => 8,
            _ => {
                return None;
            }
        };

        let bytes: Vec<u8> = self.data.drain(0..len).collect();

        if token == CHR_FLOAT32 {
            let float: f32 = config().big_endian().deserialize(&bytes).ok()?;
            Some(Object::Float(Float::F32(float)))
        } else {
            let float: f64 = config().big_endian().deserialize(&bytes).ok()?;
            Some(Object::Float(Float::F64(float)))
        }
    }

    fn decode_int(&mut self) -> Option<Object> {
        let token = self.data.pop_front()?;

        if is_embedded_pos_int(token) {
            return Some(Object::Int(Int::I8((INT_POS_FIXED_START + token) as i8)));
        } else if is_embedded_neg_int(token) {
            return Some(Object::Int(Int::I8(
                (INT_NEG_FIXED_START - 1 - token) as i8,
            )));
        }

        let len = match token {
            CHR_INT1 => 1,
            CHR_INT2 => 2,
            CHR_INT4 => 4,
            CHR_INT8 => 8,
            _ => {
                return None;
            }
        };

        let bytes: Vec<u8> = self.data.drain(0..len).collect();

        if token == CHR_INT1 {
            let int: i8 = config().big_endian().deserialize(&bytes).ok()?;
            Some(Object::Int(Int::I8(int)))
        } else if token == CHR_INT2 {
            let int: i16 = config().big_endian().deserialize(&bytes).ok()?;
            Some(Object::Int(Int::I16(int)))
        } else if token == CHR_INT4 {
            let int: i32 = config().big_endian().deserialize(&bytes).ok()?;
            Some(Object::Int(Int::I32(int)))
        } else {
            let int: i64 = config().big_endian().deserialize(&bytes).ok()?;
            Some(Object::Int(Int::I64(int)))
        }
    }

    fn decode_bytes(&mut self) -> Option<Object> {
        let bytes = self.decode_string()?;

        match String::from_utf8(bytes.clone()) {
            Ok(s) => Some(Object::Str(s)),
            Err(_) => Some(Object::Bytes(bytes)),
        }
    }

    fn decode_string(&mut self) -> Option<Vec<u8>> {
        let token = self.data.front()?.clone();

        if is_fixed_string(token) {
            self.data.pop_front()?;
            let len = (token - STR_FIXED_START) as usize;
            Some(self.data.drain(0..len).collect())
        } else if token >= ('1' as u8) && token <= ('9' as u8) {
            let mut len = String::new();

            while *self.data.front()? as char != ':' {
                len.push(self.data.pop_front()? as char);
            }
            self.data.pop_front()?;

            let len = len.parse::<usize>().ok()?;
            Some(self.data.drain(0..len).collect())
        } else {
            None
        }
    }
}

fn is_map(token: u8) -> bool {
    token == CHR_DICT
}

fn is_fixed_map(token: u8) -> bool {
    DICT_FIXED_START <= token && token < (DICT_FIXED_START + DICT_FIXED_COUNT)
}

fn is_list(token: u8) -> bool {
    token == CHR_LIST
}

fn is_fixed_list(token: u8) -> bool {
    LIST_FIXED_START <= token
        && (token as u16) < ((LIST_FIXED_START as u16) + (LIST_FIXED_COUNT as u16))
}

fn is_num(token: u8) -> bool {
    token == CHR_INT
}

fn is_bool(token: u8) -> bool {
    token == CHR_TRUE || token == CHR_FALSE
}

fn is_float(token: u8) -> bool {
    token == CHR_FLOAT32 || token == CHR_FLOAT64
}

fn is_any_int(token: u8) -> bool {
    is_num(token) || is_int(token) || is_embedded_pos_int(token) || is_embedded_neg_int(token)
}

fn is_int(token: u8) -> bool {
    token == CHR_INT1 || token == CHR_INT2 || token == CHR_INT4 || token == CHR_INT8
}

fn is_embedded_pos_int(token: u8) -> bool {
    INT_POS_FIXED_START <= token && token < (INT_POS_FIXED_START + INT_POS_FIXED_COUNT)
}

fn is_embedded_neg_int(token: u8) -> bool {
    INT_NEG_FIXED_START <= token && token < (INT_NEG_FIXED_START + INT_NEG_FIXED_COUNT)
}

fn is_fixed_string(token: u8) -> bool {
    STR_FIXED_START <= token && token < (STR_FIXED_START + STR_FIXED_COUNT)
}
