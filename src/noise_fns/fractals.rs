use crate::{
    generators::Perlin,
    transforms::{PointTransform, UniformScale},
    NoiseFn, SamplePoint, Seedable,
};
use rand::{Rng, SeedableRng};

pub const DEFAULT_PERSISTENCE: f64 = 0.5;
pub const DEFAULT_ATTENUATION: f64 = 2.0;
pub const DEFAULT_LACUNARITY: f64 = std::f64::consts::PI * 2.0 / 3.0;

/// Structs implementing this trait can be used to combine the result of multiple noise functions.
pub trait LayerBlender {
    /// Combine the values into a single value, assuming the slice represents values taken from
    /// a series of noise functions, with the first element representing data collected from the
    /// first layer of noise. There must always be at least one layer.
    fn blend(&self, layer_values: &[f64]) -> f64;
}

/// This trait is implemented for LayerBlenders that have a value indicating how much each
/// successive layer should be reduced in amplitude.
pub trait ModifiablePersistence: LayerBlender {
    fn set_persistence(&mut self, persistence: f64);
}

/// This trait is implemented for LayerBlenders that have a value indicating how much each
/// successive layer should be reduced in amplitude.
pub trait ModifiableAttenuation: LayerBlender {
    fn set_attenuation(&mut self, attenuation: f64);
}

/// This is basically a derive macro.
macro_rules! impl_mp {
    ($name:ident) => {
        impl ModifiablePersistence for $name {
            fn set_persistence(&mut self, persistence: f64) {
                self.persistence = persistence;
            }
        }
    };
}
macro_rules! impl_ma {
    ($name:ident) => {
        impl ModifiableAttenuation for $name {
            fn set_attenuation(&mut self, attenuation: f64) {
                self.attenuation = attenuation;
            }
        }
    };
}

/// A blender which multiplies each successive layer by a fixed value called 'persistence'.
#[derive(Clone, Copy, Debug)]
pub struct HomogenousBlender {
    /// Multiplier for the amplitude of each successive layer of noise.
    pub persistence: f64,
}

impl HomogenousBlender {
    pub fn new(persistence: f64) -> Self {
        Self { persistence }
    }
}

impl Default for HomogenousBlender {
    fn default() -> Self {
        Self {
            persistence: DEFAULT_PERSISTENCE,
        }
    }
}

impl LayerBlender for HomogenousBlender {
    fn blend(&self, layer_values: &[f64]) -> f64 {
        debug_assert!(layer_values.len() > 0);
        // Start with the first layer.
        let mut result = layer_values[0];
        // Later layers will have reduced amplitude.
        let mut amplitude = self.persistence;
        for value in layer_values {
            // Add the next layer of noise.
            result += *value * amplitude;
            // Reduce the amplitude for the following layer.
            amplitude *= self.persistence;
        }
        result
    }
}

impl_mp!(HomogenousBlender);

/// A blender which multiplies each successive layer by a fixed value called 'persistence' as well
/// as the combined value of all the layers before it.
#[derive(Clone, Copy, Debug)]
pub struct HeterogenousBlender {
    /// Multiplier for the amplitude of each successive layer of noise.
    pub persistence: f64,
}

impl HeterogenousBlender {
    pub fn new(persistence: f64) -> Self {
        Self { persistence }
    }
}

impl Default for HeterogenousBlender {
    fn default() -> Self {
        Self {
            persistence: DEFAULT_PERSISTENCE,
        }
    }
}

impl LayerBlender for HeterogenousBlender {
    fn blend(&self, layer_values: &[f64]) -> f64 {
        debug_assert!(layer_values.len() > 0);
        // Start with the first layer.
        let mut result = layer_values[0];
        // Later layers will have reduced amplitude.
        let mut amplitude = self.persistence;
        for value in layer_values {
            // Add the next layer of noise.
            result += *value * amplitude * result;
            // Reduce the amplitude for the following layer.
            amplitude *= self.persistence;
        }
        result
    }
}

impl_mp!(HeterogenousBlender);

/// A blender where the output of each layer is modified by
/// an absolute-value function. Modifying the layer values in this way
/// produces ridge-like formations.
///
/// The values output from this blender will usually range from -1.0 to 1.0 with
/// default values for the parameters, but there are no guarantees that all
/// output values will exist within this range. If the parameters are modified
/// from their defaults, then the output will need to be scaled to remain in
/// the [-1,1] range.
///
/// Ridged-multifractal noise is often used to generate craggy mountainous
/// terrain or marble-like textures.
#[derive(Clone, Copy, Debug)]
pub struct RidgedBlender {
    /// How much to dampen higher frequencies on points of lower magnitude.
    pub attenuation: f64,
    /// Multiplier for the amplitude of each successive layer of noise.
    pub persistence: f64,
}

impl RidgedBlender {
    pub fn new(attenuation: f64, persistence: f64) -> Self {
        Self {
            attenuation,
            persistence,
        }
    }
}

impl Default for RidgedBlender {
    fn default() -> Self {
        Self {
            attenuation: DEFAULT_ATTENUATION,
            persistence: DEFAULT_PERSISTENCE,
        }
    }
}

impl LayerBlender for RidgedBlender {
    fn blend(&self, layer_values: &[f64]) -> f64 {
        debug_assert!(layer_values.len() > 0);
        // Start with the first layer.
        let mut result = layer_values[0];
        // Later layers will have reduced amplitude.
        let mut amplitude = self.persistence;
        let mut weight = 1.0;
        for value in layer_values {
            // Make the ridges.
            let value = 1.0 - value.abs();
            // Square the signal to increase the sharpness of the ridges.
            // Apply the weighting from the previous octave to the signal.
            // Larger values have higher weights, producing sharp points along
            // the ridges.
            let value = value * value * weight;
            // Weight successive contributions by the previous signal.
            weight = value / self.attenuation;
            // Clamp the weight to [0,1] to prevent the result from diverging.
            weight = weight.clamp(0.0, 1.0);
            // Scale the amplitude appropriately for this frequency.
            // Add the signal to the result.
            result += value * amplitude;
            // Reduce the amplitude for the following layer.
            amplitude *= self.persistence;
        }
        result
    }
}

impl_mp!(RidgedBlender);
impl_ma!(RidgedBlender);

/// A noise function which is built up of multiple layers of a simpler noise function.
///
/// A transform is applied repeatedly for each successive layer that is used in the
/// calculation. For example, a transform that increases the scale of noise would
/// result in each layer having a higher frequency than the last. A transform that
/// rotates the noise would result in each layer being offset a fixed amount
/// compared to the layer before it.
///
/// A `LayerBlender` is used to combine the values from each layer into a final value. The simplest
/// available is `HomogenousBlender`, which gives each layer a successively smaller amplitude.
///
/// There are several type aliases which can be used to easily construct helpful fractal noise
/// functions:
/// ```rust
/// let
/// ```
#[derive(Clone, Debug)]
pub struct Fractal<
    Blender: LayerBlender = HomogenousBlender,
    BaseFunction: Seedable = Perlin,
    Transform = UniformScale<f64>,
> {
    layers: Vec<BaseFunction>,
    transform: Transform,
    blender: Blender,
    seed: u32,
}

// Aliases for commonly used fractal noise types.

/// Fractal noise based on stacking Perlin noise layers. This is the same algorithm commonly labeled
/// as 'perlin noise' in popular applications. (The Perlin struct itself only generates one layer
/// of noise and therefore looks very blobby and low-res on its own.)
pub type FractalPerlin = Fractal;

/// Fractal noise with heterogenous blending. The effect of this is that in areas near zero, higher
/// frequencies will be heavily damped, resulting in the terrain remaining
/// smooth. As the value moves further away from zero, higher frequencies will
/// not be as damped and thus will grow more jagged as iteration progresses.
pub type HeteroFractal = Fractal<HeterogenousBlender>;

impl<B, F> Default for Fractal<B, F, UniformScale<f64>>
where
    B: Default + LayerBlender,
    F: Seedable + Default,
{
    fn default() -> Self {
        Self::new(
            Self::DEFAULT_LAYERS,
            // Default lacunarity
            UniformScale::new(DEFAULT_LACUNARITY),
            Default::default(),
        )
    }
}

impl<B, F, T> Fractal<B, F, T>
where
    F: Seedable,
    B: LayerBlender,
{
    /// The default seed for the first layer of noise. Chosen by fair dice roll, guaranteed to be
    /// random.
    pub const DEFAULT_SEED: u32 = 0xD078_6B3E;
    pub const DEFAULT_LAYERS: u32 = 6;
    pub const MAX_LAYERS: usize = 32;

    pub fn new(layers: u32, transform: T, blender: B) -> Self
    where
        F: Default,
    {
        let seed = Self::DEFAULT_SEED;
        // Using an rng to create the seeds ensures that similar seeds produce
        // different results.
        let mut seed_gen = rand_xorshift::XorShiftRng::seed_from_u64(seed as _);
        let layers = (0..layers)
            .map(|_| F::default().with_seed(seed_gen.gen()))
            .collect();
        Self {
            layers,
            transform,
            blender,
            seed,
        }
    }

    /// Returns this fractal noise function but modified to use the specified noise function
    /// duplicated `layers` times with different seeds assigned to each function.
    pub fn with_function<NewF>(self, function_template: NewF) -> Fractal<B, NewF, T>
    where
        NewF: Seedable + Clone,
    {
        debug_assert!(self.layers.len() > 0);
        let mut seed_gen = rand_xorshift::XorShiftRng::seed_from_u64(self.seed as _);
        let layers = (0..self.layers.len())
            .map(|_| function_template.clone().with_seed(seed_gen.gen()))
            .collect();
        Fractal {
            layers,
            blender: self.blender,
            seed: self.seed,
            transform: self.transform,
        }
    }

    /// Just like `with_function` but creates the functions with `Seedable::from_seed` instead of
    /// cloning a template.  
    pub fn with_function_default<NewF>(self) -> Fractal<B, NewF, T>
    where
        NewF: Seedable + Clone + Default,
    {
        self.with_function(NewF::default())
    }

    pub fn with_layers(self, layers: usize) -> Self
    where
        F: Clone,
    {
        assert!(layers > 0);
        let current_num_layers = self.layers.len();
        let layers = if current_num_layers == layers {
            self.layers
        } else if current_num_layers > layers {
            let mut o = self.layers;
            drop(o.split_off(layers));
            o
        } else {
            let mut o = self.layers;
            debug_assert!(o.len() > 0);
            let template = o.first().unwrap();
            let mut seed_gen = rand_xorshift::XorShiftRng::seed_from_u64(self.seed as _);
            // This ensures that the new layers will get the same seeds as if they were created
            // all at once in new() or similar.
            for _ in 0..current_num_layers {
                seed_gen.gen::<u32>();
            }
            let mut next = Vec::new();
            for _ in current_num_layers..layers {
                next.push(template.clone().with_seed(seed_gen.gen()));
            }
            o.append(&mut next);
            o
        };
        Self { layers, ..self }
    }

    /// Returns this fractal modified to use the provided point transformer repeatedly for each
    /// layer. For example, the first layer will have no transformation applied, while the fourth
    /// layer will have the transformation applied three times.
    pub fn with_transform<NewT>(self, transform: NewT) -> Fractal<B, F, NewT> {
        Fractal {
            transform,
            blender: self.blender,
            layers: self.layers,
            seed: self.seed,
        }
    }

    /// Returns this fractal modified to use the provided layer blender to combine the values
    /// produced by all noise layers.
    pub fn with_layer_blender<NewB: LayerBlender>(self, blender: NewB) -> Fractal<NewB, F, T> {
        Fractal {
            blender,
            layers: self.layers,
            seed: self.seed,
            transform: self.transform,
        }
    }
}

impl<F, B, E> Fractal<B, F, UniformScale<E>>
where
    F: Seedable,
    B: LayerBlender,
{
    /// Returns this fractal modified to scale each layer by the provided amount.
    pub fn with_lacunarity(self, lacunarity: E) -> Self {
        self.with_transform(UniformScale::new(lacunarity))
    }
}

impl<B, F, T> Fractal<B, F, T>
where
    F: Seedable,
    B: LayerBlender + ModifiablePersistence,
{
    /// Returns this fractal modified so that the amplitude of each successive layer is reduced
    /// by the given amount.
    pub fn with_persistence(self, persistence: f64) -> Self {
        let mut this = self;
        this.blender.set_persistence(persistence);
        this
    }
}

impl<B, F, T> Fractal<B, F, T>
where
    F: Seedable,
    B: LayerBlender + ModifiableAttenuation,
{
    /// Returns this fractal modified so that lower amplitude values will reduce the strength of
    /// successive layers using the given value. Higher values mean stronger reduction.
    pub fn with_attenuation(self, attenuation: f64) -> Self {
        let mut this = self;
        this.blender.set_attenuation(attenuation);
        this
    }
}

impl<P, B, F, T> NoiseFn<P> for Fractal<B, F, T>
where
    P: SamplePoint + Clone,
    F: Seedable + NoiseFn<P>,
    T: PointTransform<P>,
    B: LayerBlender,
{
    fn get(&self, point: P) -> f64 {
        let mut point = point;
        let values: Vec<f64> = self
            .layers
            .iter()
            .map(move |layer| {
                // Get the value for this layer.
                let v = layer.get(point.clone());
                // Apply the transform for the next layer.
                point = self.transform.transform(point.clone());
                v
            })
            .collect();
        debug_assert!(values.len() > 0);
        self.blender.blend(&values[..])
    }
}

impl<B, F, T> Seedable for Fractal<B, F, T>
where
    F: Seedable,
    B: LayerBlender,
{
    /// Changes the seeds of all layers based on the provided seed.
    fn with_seed(self, seed: u32) -> Self {
        // I just don't like putting bare 'mut' in public function headers.
        let this = self;
        let mut seed_gen = rand_xorshift::XorShiftRng::seed_from_u64(seed as _);
        let layers = this
            .layers
            .into_iter()
            .map(|layer| layer.with_seed(seed_gen.gen()))
            .collect();
        Self {
            layers,
            blender: this.blender,
            seed: this.seed,
            transform: this.transform,
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}
