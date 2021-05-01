use num_traits::Num;

use crate::NoiseFn;

/// Noise function that outputs concentric cylinders.
///
/// This noise function outputs concentric cylinders centered on the origin. The
/// cylinders are oriented along the z axis similar to the concentric rings of
/// a tree. Each cylinder extends infinitely along the z axis.
#[derive(Clone, Copy, Debug)]
pub struct Cylinders;

impl Cylinders {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Cylinders {
    fn default() -> Self {
        Self::new()
    }
}

impl<E, const N: usize> NoiseFn<[E; N]> for Cylinders
where
    E: Num + Copy + Into<f64>,
{
    fn get(&self, point: [E; N]) -> f64 {
        // Scale the inputs by the frequency.
        let x = point[0].into();
        let y = point[1].into();

        // Calculate the distance of the point from the origin.
        let dist_from_center = (x.powi(2) + y.powi(2)).sqrt();

        let dist_from_smaller_sphere = dist_from_center - dist_from_center.floor();
        let dist_from_larger_sphere = 1.0 - dist_from_smaller_sphere;
        let nearest_dist = dist_from_smaller_sphere.min(dist_from_larger_sphere);

        // Shift the result to be in the -1.0 to +1.0 range.
        1.0 - (nearest_dist * 4.0)
    }
}
