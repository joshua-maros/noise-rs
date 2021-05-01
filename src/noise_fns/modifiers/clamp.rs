use crate::{SamplePoint, noise_fns::NoiseFn};

/// Noise function that clamps the output value from the source function to a
/// range of values.
pub struct Clamp<Source> {
    /// Outputs a value.
    pub source: Source,

    /// Bound of the clamping range. Default is -1.0 to 1.0.
    pub bounds: (f64, f64),
}

impl<Source> Clamp<Source> {
    pub fn new(source: Source) -> Self {
        Self {
            source,
            bounds: (-1.0, 1.0),
        }
    }

    pub fn with_lower_bound(self, lower_bound: f64) -> Self {
        Self {
            bounds: (lower_bound, self.bounds.1),
            ..self
        }
    }

    pub fn with_upper_bound(self, upper_bound: f64) -> Self {
        Self {
            bounds: (self.bounds.0, upper_bound),
            ..self
        }
    }

    pub fn with_bounds(self, lower_bound: f64, upper_bound: f64) -> Self {
        Self {
            bounds: (lower_bound, upper_bound),
            ..self
        }
    }
}

impl<P, Source> NoiseFn<P> for Clamp<Source> 
where P: SamplePoint, Source: NoiseFn<P>
{
    fn get(&self, point: P) -> f64 {
        let value = self.source.get(point);

        value.clamp(self.bounds.0, self.bounds.1)
    }
}
