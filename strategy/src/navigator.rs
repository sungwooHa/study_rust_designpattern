use crate::transportation_strategy::transportation::Transportation;

pub struct Navigator<T: Transportation> {
    transportation_strategy: T,
}

impl<T: Transportation> Navigator<T> {
    pub fn new(transportation_strategy: T) -> Self {
        Self { transportation_strategy }
    }

    pub fn route(&self, from: &str, to: &str) {
        self.transportation_strategy.build_route(from, to);
    }
}