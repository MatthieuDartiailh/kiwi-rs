//!
//!
//!

///
#[inline]
pub fn create(a: f64, b: f64, c: f64, weight: f64) -> f64 {
    let mut result = 0.0;
    result += f64::max(0.0, f64::min(1000.0, a * weight)) * 1e6;
    result += f64::max(0.0, f64::min(1000.0, b * weight)) * 1e3;
    result += f64::max(0.0, f64::min(1000.0, c * weight)) * 1.0;
    result
}

/// create(1000.0, 1000.0, 1000.0, 1.0);
pub const REQUIRED: f64 = 1e9 + 1e6 + 1e3;

/// create(1.0, 0.0, 0.0, 1.0);
pub const STRONG: f64 = 1e6;

/// create(0.0, 1.0, 0.0, 1.0);
pub const MEDIUM: f64 = 1e3;

///create(0.0, 0.0, 1000.0, 1.0)
pub const WEAK: f64 = 1.0;

///
#[inline]
pub fn clip(strength: f64) -> f64 {
    f64::max(0.0, f64::min(REQUIRED, strength))
}
