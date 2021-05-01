use crate::{noise_fns::NoiseFn, SamplePoint};

/// Noise function that negates the output value from the source function.
pub struct Negate<Source> {
    /// Outputs a value.
    pub source: Source,
}

impl<Source> Negate<Source> {
    pub fn new(source: Source) -> Self {
        Negate { source }
    }
}

impl<P, Source> NoiseFn<P> for Negate<Source>
where
    P: SamplePoint,
    Source: NoiseFn<P>,
{
    fn get(&self, point: P) -> f64 {
        -self.source.get(point)
    }
}
