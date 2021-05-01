use crate::{NoiseFn, SamplePoint};

/// Noise function that outputs the absolute value of the output value from the
/// source function.
pub struct Abs<Source> {
    /// Outputs a value.
    pub source: Source,
}

impl<Source> Abs<Source> {
    pub fn new(source: Source) -> Self {
        Self { source }
    }
}

impl<P, Source> NoiseFn<P> for Abs<Source>
where
    P: SamplePoint,
    Source: NoiseFn<P>,
{
    fn get(&self, point: P) -> f64 {
        (self.source.get(point)).abs()
    }
}
