use num_traits::Num;

use crate::{
    math::SamplePoint,
    transforms::{PointTransform, Transformed, UniformScale},
};

pub mod cache;
pub mod combiners;
pub mod fractals;
pub mod generators;
pub mod modifiers;
pub mod selectors;
pub mod transformers;

/// Base trait for noise functions.
///
/// A noise function is a object that calculates and outputs a value given a
/// n-Dimensional input value, where n is (2,3,4).
///
/// Each type of noise function uses a specific method to calculate an output
/// value. Some of these methods include:
///
/// * Calculating a value using a coherent-noise function or some other
///     mathematical function.
/// * Mathematically changing the output value from another noise function
///     in various ways.
/// * Combining the output values from two noise functions in various ways.
pub trait NoiseFn<P: SamplePoint> {
    fn get(&self, point: P) -> f64;

    fn transformed<T>(self, transform: T) -> Transformed<Self, T>
    where
        Self: Sized,
        T: PointTransform<P>,
    {
        Transformed {
            source: self,
            transform,
        }
    }

    fn scaled<S: Num>(self, factor: S) -> Transformed<Self, UniformScale<S>>
    where
        Self: Sized,
    {
        Transformed {
            source: self,
            transform: UniformScale::new(factor),
        }
    }
}

impl<'a, P: SamplePoint, M: NoiseFn<P>> NoiseFn<P> for &'a M {
    #[inline]
    fn get(&self, point: P) -> f64 {
        M::get(*self, point)
    }
}

/// Trait for functions that require a seed before generating their values
pub trait Seedable {
    /// Set the seed for the function implementing the `Seedable` trait
    fn with_seed(self, seed: u32) -> Self;

    /// Getter to retrieve the seed from the function
    fn seed(&self) -> u32;
}
