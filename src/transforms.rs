use crate::{math::SamplePointMath, SamplePoint};

pub trait PointTransform<P: SamplePoint> {
    fn transform(&self, point: P) -> P;
}

/// A `PointTransform` which scales points uniformly across all axes.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct UniformScale<T>(T);

impl<T> UniformScale<T> {
    pub fn new(scale: T) -> Self {
        Self(scale)
    }
}

impl<T, P> PointTransform<P> for UniformScale<T>
where
    P: SamplePoint<Element = T>,
{
    fn transform(&self, point: P) -> P {
        point.mul_scalar(self.0)
    }
}
