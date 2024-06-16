mod encode;
mod decode;
mod decode;

use bytes::BytesMut;
use std::collections::{HashMap, HashSet};


pub trait RespEncode {
    fn encode(&self) -> Vec<u8>;
}

pub trait RespDecode {
    fn decode(&self) -> Result<RespFrame, String>;
}

impl RespDecode for BytesMut {
    fn decode(&self) -> Result<RespFrame, String> {
        todo!()
    }
}

// redis resp frame
pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(Vec<u8>),
    NullBulkString(RespNullBulkString),
    Array(Vec<RespFrame>),
    Null(RespNull),
    NullArray(RespNullArray),
    Boolean(bool),
    Double(f64),
    Map(HashMap<String, RespFrame>),
    Set(HashSet<RespFrame>)
}

pub struct SimpleString(String);
pub struct SimpleError(String);
pub struct RespNull;
pub struct RespNullArray;
pub struct Null;
pub struct RespNullBulkString;