use crate::data_types::Float;

pub trait FloatSummator {
    fn execute(value1: Float, value2: Float) -> Float
}
