/// A struct of values array.
///
/// It's going to be an output of `Visitor` trait which is defined for the type
/// in `visitor.rs`.
#[derive(Default, Debug)]
pub struct TwoValuesArray {
    pub ab: [i32; 2],
}
