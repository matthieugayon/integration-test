use crate::h_slider::normal::{Normal, NormalParam};

pub const DEFAULT_QUANTIZED_SPEED_INDEX: usize = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeedMode {
    Quantized,
    Unquantized
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpeedValue {
    Quantized(usize),
    Unquantized(f32)
}

impl Default for SpeedValue {
    fn default() -> Self {
        SpeedValue::Quantized(DEFAULT_QUANTIZED_SPEED_INDEX)
    }
}

pub struct QuantizedSpeedValue {
    pub numerator: f32,
    pub denominator: f32,
    pub text: &'static str,
    pub mark_weight: Option<MarkWeight>,
    pub text_mark: Option<&'static str>
}

pub enum MarkWeight {
    Normal,
    Bold
}

pub const QUANTIZED_SPEEDS_LEN: usize = 15;
pub const QUANTIZED_SPEEDS: [QuantizedSpeedValue; QUANTIZED_SPEEDS_LEN] = [
    QuantizedSpeedValue {
        numerator: 1.,
        denominator: 8.,
        text: "1/8",
        mark_weight: Some(MarkWeight::Bold),
        text_mark: Some("÷8")
    },
    QuantizedSpeedValue {
        numerator: 1.,
        denominator: 7.,
        text: "1/7",
        mark_weight: Some(MarkWeight::Normal),
        text_mark: None
    },
    QuantizedSpeedValue {
        numerator: 1.,
        denominator: 6.,
        text: "1/6",
        mark_weight: Some(MarkWeight::Normal),
        text_mark: None
    },
    QuantizedSpeedValue {
        numerator: 1.,
        denominator: 5.,
        text: "1/5",
        mark_weight: Some(MarkWeight::Normal),
        text_mark: None
    },
    QuantizedSpeedValue {
        numerator: 1.,
        denominator: 4.,
        text: "1/4",
        mark_weight: Some(MarkWeight::Normal),
        text_mark: None
    },
    QuantizedSpeedValue {
        numerator: 1.,
        denominator: 3.,
        text: "1/3",
        mark_weight: Some(MarkWeight::Normal),
        text_mark: None
    },
    QuantizedSpeedValue {
        numerator: 1.,
        denominator: 2.,
        text: "1/2",
        mark_weight: Some(MarkWeight::Normal),
        text_mark: Some("÷2")
    },
    QuantizedSpeedValue {
        numerator: 1.,
        denominator: 1.,
        text: "1",
        mark_weight: Some(MarkWeight::Bold),
        text_mark: Some("1")
    },
    QuantizedSpeedValue {
        numerator: 2.,
        denominator: 1.,
        text: "2",
        mark_weight: Some(MarkWeight::Normal),
        text_mark: Some("x2")
    },
    QuantizedSpeedValue {
        numerator: 3.,
        denominator: 1.,
        text: "3",
        mark_weight: Some(MarkWeight::Normal),
        text_mark: None
    },
    QuantizedSpeedValue {
        numerator: 4.,
        denominator: 1.,
        text: "x4",
        mark_weight: Some(MarkWeight::Normal),
        text_mark: None
    },
    QuantizedSpeedValue {
        numerator: 5.,
        denominator: 1.,
        text: "5",
        mark_weight: Some(MarkWeight::Normal),
        text_mark: None
    },
    QuantizedSpeedValue {
        numerator: 6.,
        denominator: 1.,
        text: "6",
        mark_weight: Some(MarkWeight::Normal),
        text_mark: None
    },
    QuantizedSpeedValue {
        numerator: 7.,
        denominator: 1.,
        text: "7",
        mark_weight: Some(MarkWeight::Normal),
        text_mark: None
    },
    QuantizedSpeedValue {
        numerator: 8.,
        denominator: 1.,
        text: "8",
        mark_weight: Some(MarkWeight::Bold),
        text_mark: Some("x8")
    }
];

/// A [`NormalParam`] that defines a continuous log2 range of `f32` speed
/// values centered around 1
///
#[derive(Debug, Copy, Clone)]
pub struct SpeedRange {
    min: f32,
    max: f32,
    octave_span: f32,
    octave_span_recip: f32,
    min_speed: f32,
    speed_span_recip: f32
}

impl SpeedRange {
    pub fn new(min: f32, max: f32) -> Self {
        assert!(max > min);

        let mut min = min;
        if min < 0.125 {
            min = 0.125;
        }

        let mut max = max;
        if max > 8. {
            max = 8.;
        }

        let min_octave = speed_to_octave(min);
        let max_octave = speed_to_octave(max);

        let octave_span = max_octave - min_octave;
        let octave_span_recip = 1.0 / octave_span;

        let min_speed = octave_to_speed(0.);
        let max_speed = octave_to_speed(octave_span);
        let speed_span_recip = 1. / ( max_speed - min_speed);

        Self {
            min,
            max,
            octave_span,
            octave_span_recip,
            min_speed,
            speed_span_recip
        }
    }

    fn constrain(&self, value: f32) -> f32 {
        if value <= self.min {
            self.min
        } else if value >= self.max {
            self.max
        } else {
            value
        }
    }

    pub fn normal_param(&self, value: f32, default: f32) -> NormalParam {
        NormalParam {
            value: self.map_to_normal(value),
            default: self.map_to_normal(default),
        }
    }

    pub fn default_normal_param(&self) -> NormalParam {
        NormalParam {
            value: self.map_to_normal(1.),
            default: self.map_to_normal(1.),
        }
    }

    pub fn map_to_normal(&self, value: f32) -> Normal {
        let value = self.constrain(value);
        let speed_octave = speed_to_octave(value);
        ((speed_octave + (self.octave_span * 0.5)) * self.octave_span_recip).into()
        // Normal::from_clipped((speed_octave - (self.octave_span * 0.5)) * self.octave_span_recip)
    }

    pub fn unmap_to_value(&self, normal: Normal) -> f32 {
        let speed_octave = normal.as_f32() * self.octave_span;
        self.min + ((octave_to_speed(speed_octave) - self.min_speed) * self.speed_span_recip) * (self.max - self.min)
    }
}

impl Default for SpeedRange {
    fn default() -> Self {
        SpeedRange::new(0.125, 8.0)
    }
}

#[inline]
fn octave_to_speed(value: f32) -> f32 {
    2.0_f32.powf(value)
}

#[inline]
fn speed_to_octave(speed: f32) -> f32 {
    speed.log2()
}
