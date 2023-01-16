use crate::deser::{deserializer::Deserializer, visitor::Visitor};
pub struct StringDeserializer<V: Visitor> {
    visitor: V,
}

impl<V: Visitor> Deserializer<V> for StringDeserializer<V> {
    fn create(visitor: V) -> Self {
        Self { visitor }
    }

    fn parse_str(&self, input: &str) -> Result<V::Value, &'static str> {
        // In this case, in order to apply a visitor, a deserializer should do
        // some preparation. The visitor does its stuff, but it doesn't do everything.
        let input_vec = input
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        Ok(self.visitor.visit_vec(input_vec))
    }
}
