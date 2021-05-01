use crate::{
    generators::Perlin,
    transforms::{PointTransform, UniformScale},
    NoiseFn, SamplePoint, Seedable,
};

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

pub struct HomogenousBlender {
    persistence: f64,
}

impl HomogenousBlender {
    pub fn new(persistence: f64) -> Self {
        Self { persistence }
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

pub struct Fractal<
    BaseFunction: Seedable = Perlin,
    Transform = UniformScale<f64>,
    Blender: LayerBlender = HomogenousBlender,
> {
    layers: Vec<BaseFunction>,
    transform: Transform,
    blender: Blender,
    seed: u32,
}

impl<F: Seedable> Default for Fractal<F> {
    fn default() -> Self {
        Self::new(Self::DEFAULT_LAYERS)
    }
}

impl<F, T, B> Fractal<F, T, B>
where
    F: Seedable,
    B: LayerBlender,
{
    /// The default seed for the first layer of noise. Chosen by fair dice roll, guaranteed to be
    /// random.
    pub const DEFAULT_SEED: u32 = 0xD078_6B3E;
    pub const DEFAULT_LAYERS: u32 = 6;
    pub const DEFAULT_FREQUENCY: f64 = 2.0;
    pub const DEFAULT_LACUNARITY: f64 = std::f64::consts::PI * 2.0 / 3.0;
    pub const DEFAULT_PERSISTENCE: f64 = 0.5;
    pub const MAX_LAYERS: usize = 32;

    pub fn new(layers: u32) -> Self {
        let seed = Self::DEFAULT_SEED;
        let layers = (0..layers)
            .map(|offset| F::from_seed(seed + offset))
            .collect();
        Self {
            layers,
            transform: UniformScale::new(Self::DEFAULT_LACUNARITY),
            blender: HomogenousBlender::new(Self::DEFAULT_PERSISTENCE),
            seed,
        }
    }

    /// Returns this fractal noise function but modified to use the specified noise function
    /// duplicated `layers` times with different seeds assigned to each function.
    pub fn with_function<NewF>(self, function_template: NewF) -> Fractal<NewF, T, B>
    where
        NewF: Seedable + Clone,
    {
        debug_assert!(self.layers.len() > 0);
        let layers = (0..self.layers.len())
            .map(|layer| {
                function_template
                    .clone()
                    .with_seed(layer + Self::DEFAULT_SEED)
            })
            .collect();
        Self { layers, ..self }
    }

    /// Just like `with_function` but creates the functions with `Seedable::from_seed` instead of
    /// cloning a template.  
    pub fn with_function_default<NewF>(self) -> Fractal<NewF, T, B>
    where
        NewF: Seedable + Clone,
    {
        self.with_function(NewF::from_seed(0))
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
            o.split_off(layers);
            o
        } else {
            let mut o = self.layers;
            debug_assert!(o.len() > 0);
            let template = o.first().unwrap();
            for seed_offset in current_num_layers..layers {
                o.push(template.clone().with_seed(Self::DEFAULT_SEED + seed_offset));
            }
            o
        };
        Self { layers, ..self }
    }

    /// Returns this fractal modified to use the provided point transformer repeatedly for each
    /// layer. For example, the first layer will have no transformation applied, while the fourth
    /// layer will have the transformation applied three times.
    pub fn with_transform<NewT>(self, transform: NewT) -> Fractal<F, NewT, B> {
        Self { transform, ..self }
    }

    /// Returns this fractal modified to use the provided layer blender to combine the values
    /// produced by all noise layers.
    pub fn with_layer_blender<NewB: LayerBlender>(self, blender: NewB) -> Fractal<F, T, NewB> {
        Self { blender, ..self }
    }

    /// Changes the seeds of all layers based on the provided seed.
    pub fn with_seed(self, seed: u32) -> Self {
        // I just don't like putting bare 'mut' in public function headers.
        let mut this = self;
        let mut seed = seed;
        for layer in &mut this.layers {
            layer = layer.with_seed(seed);
            seed += 1;
        }
        this
    }
}

impl<F, B, E> Fractal<F, UniformScale<E>, B>
where
    F: Seedable,
    B: LayerBlender,
{
    /// Returns this fractal modified to scale each layer by the provided amount.
    pub fn with_lacunarity(self, lacunarity: E) -> Self {
        self.with_transform(UniformScale::new(lacunarity))
    }
}

impl<F, T, B> Fractal<F, T, B>
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

impl<P, F, T, B> NoiseFn<P> for Fractal<F, T, B>
where
    P: SamplePoint,
    F: Seedable,
    T: PointTransform<P>,
    B: LayerBlender + ModifiablePersistence,
{
    fn get(&self, point: P) -> f64 {
        let mut point = point;
        let values: Vec<f64> = self
            .layers
            .iter()
            .map(move |layer| {
                // Get the value for this layer.
                let v = layer.get(point);
                // Apply the transform for the next layer.
                point = self.transform.transform(point);
            })
            .collect();
        debug_assert!(values.len() > 0);
        self.blender.blend(&values[..])
    }
}
