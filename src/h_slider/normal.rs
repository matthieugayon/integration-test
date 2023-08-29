//! An `f32` value that is gauranteed to be constrained to the range of
//!
//! `0.0 >= value <= 1.0`

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Normal {
    value: f32,
}

impl Default for Normal {
    fn default() -> Self {
        Self { value: 0.0 }
    }
}

impl Normal {
    pub const MIN: Self = Self { value: 0.0 };
    pub const CENTER: Self = Self { value: 0.5 };
    pub const MAX: Self = Self { value: 1.0 };

    #[inline]
    pub fn from_clipped(value: f32) -> Self {
        Self {
            value: {
                if value < 0.0 {
                    0.0
                } else if value > 1.0 {
                    1.0
                } else {
                    value
                }
            },
        }
    }

    #[inline]
    pub fn new(value: f32) -> Self {
        Self {
            value: {
                if value < 0.0 {
                    0.0
                } else if value > 1.0 {
                    1.0
                } else {
                    value
                }
            },
        }
    }

    #[inline]
    pub fn set_clipped(&mut self, value: f32) {
        *self = Normal::from_clipped(value)
    }

    #[inline]
    pub fn as_f32(&self) -> f32 {
        self.value
    }

    #[inline]
    pub fn as_f32_inv(&self) -> f32 {
        1.0 - self.value
    }

    #[inline]
    pub fn scale(&self, scalar: f32) -> f32 {
        self.value * scalar
    }

    #[inline]
    pub fn scale_inv(&self, scalar: f32) -> f32 {
        (1.0 - self.value) * scalar
    }
}

impl From<f32> for Normal {
    fn from(value: f32) -> Self {
        Normal::new(value)
    }
}


impl From<Normal> for f32 {
    fn from(normal: Normal) -> f32 {
        normal.value
    }
}


/// A paramater that contains a normalized `value` and a `default_value`.
///
/// The values are stored as the [`Normal`] type.

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NormalParam {
    pub value: Normal,
    pub default: Normal,
}

impl Default for NormalParam {
    fn default() -> Self {
        Self {
            value: Normal::MIN,
            default: Normal::MIN,
        }
    }
}

impl NormalParam {
    #[inline]
    pub fn update(&mut self, normal: Normal) {
        self.value = normal;
    }
}

