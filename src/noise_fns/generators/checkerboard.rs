use num_traits::ToPrimitive;

use crate::{NoiseFn, SamplePoint};

/// Noise function that outputs a checkerboard pattern.
///
/// This noise function can take one input, size, and outputs 2<sup>size</sup>-sized
/// blocks of alternating values. The values of these blocks alternate between
/// -1.0 and 1.0.
///
/// This noise function is not very useful by itself, but it can be used for
/// debugging purposes.
#[derive(Clone, Copy, Debug)]
pub struct Checkerboard {
    // Controls the size of the block in 2^(size).
    size: u64,
}

impl Checkerboard {
    const DEFAULT_SIZE: u64 = 0;

    /// Controls the size of the block in 2^(size) units.
    pub fn new(size: u64) -> Self {
        Self { size: 1 << size }
    }

    pub fn with_size(self, size: u64) -> Self {
        Self { size: 1 << size }
    }

    pub fn size(self) -> u64 {
        self.size
    }
}

impl Default for Checkerboard {
    fn default() -> Self {
        Self {
            size: 1 << Checkerboard::DEFAULT_SIZE,
        }
    }
}

impl<P> NoiseFn<P> for Checkerboard
where
    P: SamplePoint,
    P::Element: ToPrimitive,
{
    fn get(&self, point: P) -> f64 {
        let result = point
            .into_raw()
            .iter()
            .map(|&a| a.to_u64().unwrap() as u64)
            .fold(0, |a, b| (a & self.size) ^ (b & self.size));

        if result > 0 {
            -1.0
        } else {
            1.0
        }
    }
}
