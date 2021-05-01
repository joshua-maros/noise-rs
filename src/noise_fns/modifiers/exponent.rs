use crate::{math::scale_shift, NoiseFn, SamplePoint};

/// Noise function that maps the output value from the source function onto an
/// exponential curve.
///
/// Because most noise functions will output values that range from -1.0 to 1.0,
/// this noise function first normalizes the output value (the range becomes 0.0
/// to 1.0), maps that value onto an exponential curve, then rescales that
/// value back to the original range.
pub struct Exponent<Source> {
    /// Outputs a value.
    pub source: Source,

    /// Exponent to apply to the output value from the source function. Default
    /// is 1.0.
    pub exponent: f64,
}

impl<Source> Exponent<Source> {
    pub fn new(source: Source) -> Self {
        Self {
            source,
            exponent: 1.0,
        }
    }

    pub fn with_exponent(self, exponent: f64) -> Self {
        Self { exponent, ..self }
    }
}

impl<P, Source> NoiseFn<P> for Exponent<Source>
where
    P: SamplePoint,
    Source: NoiseFn<P>,
{
    fn get(&self, point: P) -> f64 {
        let mut value = self.source.get(point);
        value = (value + 1.0) / 2.0;
        value = value.abs();
        value = value.powf(self.exponent);
        scale_shift(value, 2.0)
    }
}
