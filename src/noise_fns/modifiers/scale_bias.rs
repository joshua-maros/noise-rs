use crate::{NoiseFn, SamplePoint};

/// Noise function that applies a scaling factor and a bias to the output value
/// from the source function.
///
/// The function retrieves the output value from the source function, multiplies
/// it with the scaling factor, adds the bias to it, then outputs the value.
pub struct ScaleBias<Source> {
    /// Outputs a value.
    pub source: Source,

    /// Scaling factor to apply to the output value from the source function.
    /// The default value is 1.0.
    pub scale: f64,

    /// Bias to apply to the scaled output value from the source function.
    /// The default value is 0.0.
    pub bias: f64,
}

impl<Source> ScaleBias<Source> {
    pub fn new(source: Source) -> Self {
        Self {
            source,
            scale: 1.0,
            bias: 0.0,
        }
    }

    pub fn with_scale(self, scale: f64) -> Self {
        Self { scale, ..self }
    }

    pub fn with_bias(self, bias: f64) -> Self {
        Self { bias, ..self }
    }
}

impl<P, Source> NoiseFn<P> for ScaleBias<Source>
where
    P: SamplePoint,
    Source: NoiseFn<P>,
{
    #[cfg(not(target_os = "emscripten"))]
    fn get(&self, point: P) -> f64 {
        (self.source.get(point)).mul_add(self.scale, self.bias)
    }

    #[cfg(target_os = "emscripten")]
    fn get(&self, point: P) -> f64 {
        (self.source.get(point) * self.scale) + self.bias
    }
}
