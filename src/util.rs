//!
//!

/// Check if a floating point value is close to zero
pub fn near_zero(value: f64) -> bool {
    let eps = 1.0e-8;
    value.abs() < eps
}

#[cfg(test)]
mod test {

    use super::near_zero;

    #[test]
    fn test_near_zero() {
        assert!(near_zero(1e-9));
        assert!(!near_zero(1.0));
    }
}
