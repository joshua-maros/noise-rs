use crate::{NoiseFn, SamplePoint, Seedable};
use num_traits::Num;

pub trait PointTransform<P: SamplePoint>: Default {
    fn transform(&self, point: P) -> P;
}

/// A `PointTransform` which scales points uniformly across all axes.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct UniformScale<T> {
    pub scale: T,
}

impl<T> UniformScale<T> {
    pub fn new(scale: T) -> Self {
        Self { scale }
    }
}

impl<T: Num> Default for UniformScale<T> {
    fn default() -> Self {
        Self { scale: T::one() }
    }
}

impl<T: Num + Copy, const N: usize> PointTransform<[T; N]> for UniformScale<T> {
    fn transform(&self, point: [T; N]) -> [T; N] {
        point.mul_scalar(self.scale)
    }
}

#[derive(Clone, Debug)]
pub struct Transformed<Source, Transform> {
    pub source: Source,
    pub transform: Transform,
}

impl<P, S, T> NoiseFn<P> for Transformed<S, T>
where
    P: SamplePoint,
    S: NoiseFn<P>,
    T: PointTransform<P>,
{
    fn get(&self, point: P) -> f64 {
        self.source.get(self.transform.transform(point))
    }
}

impl<Source, Transform> Seedable for Transformed<Source, Transform>
where
    Source: Seedable,
{
    fn with_seed(self, seed: u32) -> Self {
        Self {
            source: self.source.with_seed(seed),
            transform: self.transform,
        }
    }

    fn seed(&self) -> u32 {
        self.source.seed()
    }
}
