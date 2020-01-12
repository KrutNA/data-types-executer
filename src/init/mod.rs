pub mod float_initializer;
pub use float_initializer::FloatInitializer;

pub fn parse_binary(binary: &str) -> i64 {
    i64::from_str_radix(binary, 2).unwrap()
}
