use std::collections::HashMap;

#[derive(Debug)]
pub enum Object {
    Map(HashMap<String, Object>),
    List(Vec<Object>),
    Bool(bool),
    Float(Float),
    Int(Int),
    Str(String),
    Bytes(Vec<u8>),
}

#[derive(Debug)]
pub enum Float {
    F32(f32),
    F64(f64),
}

#[derive(Debug)]
pub enum Int {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}
