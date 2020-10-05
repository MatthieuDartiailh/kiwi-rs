//! Standard strengths and utility functions.
//!
//! Strength are implemented as conventional floating points number in Kiwi. This choice is made
//! for speed since in the context of GUI layout the mathematical correctness of the solution
//! is less relevant and the cost of using lexicographic order can be prohibitive. As a consequence
//! user defined strength need to be notably different from existing strength to have any noticeable
//! effect.
//!

// XXX The following function is a candidate to become a const func when the feature will be
// stabilized
/// Create a strength byt applying the proper multiplier to each component.
#[inline]
pub fn create(a: f64, b: f64, c: f64, weight: f64) -> f64 {
    let mut result = 0.0;
    result += f64::max(0.0, f64::min(1000.0, a * weight)) * 1e6;
    result += f64::max(0.0, f64::min(1000.0, b * weight)) * 1e3;
    result += f64::max(0.0, f64::min(1000.0, c * weight)) * 1.0;
    result
}

/// Strength used for absolutely required constraints.
///
/// All strength values are clipped below its value.
/// Equivalent to: create(1000.0, 1000.0, 1000.0, 1.0);
pub const REQUIRED: f64 = 1e9 + 1e6 + 1e3;

/// Strength used for strong constraints.
///
/// Equivalent to: create(1.0, 0.0, 0.0, 1.0);
pub const STRONG: f64 = 1e6;

/// Strength used for medium constraints.
///
/// Equivalent to: create(1.0, 1.0, 0.0, 1.0);
pub const MEDIUM: f64 = 1e3;

/// Strength used for medium constraints.
///
/// Equivalent to: create(0.0, 0.0, 1.0, 1.0)
pub const WEAK: f64 = 1.0;

/// Ensure a strength is positive and less than the REQUIRED strength.
#[inline]
pub fn clip(strength: f64) -> f64 {
    f64::max(0.0, f64::min(REQUIRED, strength))
}

#[cfg(test)]
mod tests {

    use super::{clip, create, MEDIUM, REQUIRED, STRONG, WEAK};

    #[test]
    fn test_create() {
        let s = create(1.0, 0.0, 0.0, 2.0);
        assert!(s > STRONG);
        let s = create(0.0, 1.0, 0.0, 2.0);
        assert!(s < STRONG);
        assert!(s > MEDIUM);
        let s = create(0.0, 0.0, 1.0, 2.0);
        assert!(s < MEDIUM);
        assert!(s > WEAK);
    }

    #[test]
    fn test_clip() {
        assert_eq!(clip(-10.0), 0.0);
        assert_eq!(clip(1.0e18), REQUIRED);
    }
}
