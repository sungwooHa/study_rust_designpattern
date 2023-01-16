pub trait Transportation {
    fn build_route(&self, from : &str, to : &str);
}
