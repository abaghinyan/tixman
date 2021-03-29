/// Cast a float with precision after decimal
///
/// * `value` - the value that we want to cast
/// * `precision` - number of elements after the decimal
pub fn cast_float(value: f64, precision: u32) -> f64 {
    (value * 10_i32.pow(precision) as f64).round() / 10_i32.pow(precision) as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cast_float_verify() {
        assert_eq!(cast_float(1.012345, 4), 1.0123);
    }
}