#![allow(unused)]

use crate::deser::visitor::Visitor;

/// `Deserializer` trait defines methods that can parse either a string or
/// a vector, it accepts a visitor which knows how to construct a new object
/// of a desired type (in our case, `TwoValuesArray` and `TwoValuesStruct`).
pub trait Deserializer<V: Visitor> {
    fn create(visitor: V) -> Self;
    fn parse_str(&self, input: &str) -> Result<V::Value, &'static str> {
        Err("parse_str is unimplemented")
    }
    fn parse_vec(&self, input: Vec<i32>) -> Result<V::Value, &'static str> {
        Err("parse_vec is unimplemented")
    }
}
