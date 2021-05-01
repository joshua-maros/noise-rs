use crate::{math::interpolate, NoiseFn, SamplePoint};

/// Noise function that outputs a weighted blend of the output values from two
/// source functions given the output value supplied by a control function.
///
/// This noise function uses linear interpolation to perform the blending
/// operation.
pub struct Blend<A, B, X> {
    /// Outputs one of the values to blend.
    pub source1: A,

    /// Outputs one of the values to blend.
    pub source2: B,

    /// Determines the weight of the blending operation. Negative values weight
    /// the blend towards the output value from the `source1` function. Positive
    /// values weight the blend towards the output value from the `source2`
    /// function.
    pub control: X,
}

impl<A, B, X> Blend<A, B, X> {
    pub fn new(source1: A, source2: B, control: X) -> Self {
        Blend {
            source1,
            source2,
            control,
        }
    }
}

impl<P, A, B, X> NoiseFn<P> for Blend<A, B, X>
where
    P: SamplePoint,
    A: NoiseFn<P>,
    B: NoiseFn<P>,
    X: NoiseFn<P>,
{
    fn get(&self, point: P) -> f64 {
        let lower = self.source1.get(point);
        let upper = self.source2.get(point);
        let control = self.control.get(point);

        interpolate::linear(lower, upper, control)
    }
}
