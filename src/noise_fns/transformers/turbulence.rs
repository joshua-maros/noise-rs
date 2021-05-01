use crate::{
    fractals::FractalPerlin,
    transforms::{Transformed, UniformScale},
    NoiseFn, Seedable,
};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

/// Noise function that randomly displaces the input value before returning the
/// output value from the source function.
///
/// _ is the pseudo-random displacement of the input value. The
/// get() method randomly displaces the coordinates of the input value before
/// retrieving the output value from the source function. To control the
/// turbulence, an application can modify its frequency, its power, and its
/// roughness.
#[derive(Clone, Debug)]
pub struct Turbulence<Source> {
    /// Source function that outputs a value.
    pub source: Source,

    /// Frequency value for the Turbulence function.
    pub frequency: f64,

    /// Controls the strength of the turbulence by affecting how much each
    /// point is moved.
    pub power: f64,

    /// Affects the roughness of the turbulence. Higher values are rougher.
    pub roughness: usize,

    seed: u32,
    distorters: [Transformed<FractalPerlin, UniformScale<f64>>; 4],
}

impl<Source> Turbulence<Source> {
    pub const DEFAULT_SEED: u32 = 0;
    pub const DEFAULT_FREQUENCY: f64 = 1.0;
    pub const DEFAULT_POWER: f64 = 1.0;
    pub const DEFAULT_ROUGHNESS: usize = 3;

    pub fn new(source: Source) -> Self {
        let seed = Self::DEFAULT_SEED;
        let mut seed_gen = XorShiftRng::seed_from_u64(seed as _);
        let frequency = Self::DEFAULT_FREQUENCY;
        let distorters = [
            NoiseFn::<[f64; 2]>::scaled(
                FractalPerlin::default().with_seed(seed_gen.gen()),
                frequency,
            ),
            NoiseFn::<[f64; 2]>::scaled(
                FractalPerlin::default().with_seed(seed_gen.gen()),
                frequency,
            ),
            NoiseFn::<[f64; 2]>::scaled(
                FractalPerlin::default().with_seed(seed_gen.gen()),
                frequency,
            ),
            NoiseFn::<[f64; 2]>::scaled(
                FractalPerlin::default().with_seed(seed_gen.gen()),
                frequency,
            ),
        ];
        Self {
            source,
            seed,
            frequency,
            power: Self::DEFAULT_POWER,
            roughness: Self::DEFAULT_ROUGHNESS,
            distorters,
        }
    }

    pub fn with_frequency(self, frequency: f64) -> Self {
        let mut this = self;
        this.frequency = frequency;
        for distorter in &mut this.distorters {
            distorter.transform.scale = frequency;
        }
        this
    }

    pub fn with_power(self, power: f64) -> Self {
        Self { power, ..self }
    }

    pub fn with_roughness(self, roughness: usize) -> Self {
        let mut this = self;
        this.roughness = roughness;
        let [a, b, c, d] = this.distorters;
        let distorters = [
            NoiseFn::<[f64; 2]>::transformed(a.source.with_layers(roughness), a.transform),
            NoiseFn::<[f64; 2]>::transformed(b.source.with_layers(roughness), b.transform),
            NoiseFn::<[f64; 2]>::transformed(c.source.with_layers(roughness), c.transform),
            NoiseFn::<[f64; 2]>::transformed(d.source.with_layers(roughness), d.transform),
        ];
        Self {
            distorters,
            frequency: this.frequency,
            power: this.power,
            roughness: this.roughness,
            seed: this.seed,
            source: this.source,
        }
    }
}

impl<Source> Seedable for Turbulence<Source> {
    fn with_seed(self, seed: u32) -> Self {
        let this = self;
        let [a, b, c, d] = this.distorters;
        let distorters = [
            a.with_seed(seed),
            b.with_seed(seed),
            c.with_seed(seed),
            d.with_seed(seed),
        ];
        Self {
            distorters,
            frequency: this.frequency,
            power: this.power,
            roughness: this.roughness,
            seed,
            source: this.source,
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

impl<Source> NoiseFn<[f64; 2]> for Turbulence<Source>
where
    Source: NoiseFn<[f64; 2]>,
{
    fn get(&self, point: [f64; 2]) -> f64 {
        // First, create offsets based on the input values to keep the sampled
        // points from being near a integer boundary. This is a result of
        // using perlin noise, which returns zero at integer boundaries.
        let x0 = point[0] + 12414.0 / 65536.0;
        let y0 = point[1] + 65124.0 / 65536.0;

        let x1 = point[0] + 26519.0 / 65536.0;
        let y1 = point[1] + 18128.0 / 65536.0;

        let x_distort = point[0] + (self.distorters[0].get([x0, y0]) * self.power);
        let y_distort = point[1] + (self.distorters[1].get([x1, y1]) * self.power);

        self.source.get([x_distort, y_distort])
    }
}

impl<Source> NoiseFn<[f64; 3]> for Turbulence<Source>
where
    Source: NoiseFn<[f64; 3]>,
{
    fn get(&self, point: [f64; 3]) -> f64 {
        // First, create offsets based on the input values to keep the sampled
        // points from being near a integer boundary. This is a result of
        // using perlin noise, which returns zero at integer boundaries.
        let x0 = point[0] + 12414.0 / 65536.0;
        let y0 = point[1] + 65124.0 / 65536.0;
        let z0 = point[2] + 31337.0 / 65536.0;

        let x1 = point[0] + 26519.0 / 65536.0;
        let y1 = point[1] + 18128.0 / 65536.0;
        let z1 = point[2] + 60943.0 / 65536.0;

        let x2 = point[0] + 53820.0 / 65536.0;
        let y2 = point[1] + 11213.0 / 65536.0;
        let z2 = point[2] + 44845.0 / 65536.0;

        let x_distort = point[0] + (self.distorters[0].get([x0, y0, z0]) * self.power);
        let y_distort = point[1] + (self.distorters[1].get([x1, y1, z1]) * self.power);
        let z_distort = point[2] + (self.distorters[2].get([x2, y2, z2]) * self.power);

        self.source.get([x_distort, y_distort, z_distort])
    }
}

impl<Source> NoiseFn<[f64; 4]> for Turbulence<Source>
where
    Source: NoiseFn<[f64; 4]>,
{
    fn get(&self, point: [f64; 4]) -> f64 {
        // First, create offsets based on the input values to keep the sampled
        // points from being near a integer boundary. This is a result of
        // using perlin noise, which returns zero at integer boundaries.
        let x0 = point[0] + 12414.0 / 65536.0;
        let y0 = point[1] + 65124.0 / 65536.0;
        let z0 = point[2] + 31337.0 / 65536.0;
        let u0 = point[3] + 57948.0 / 65536.0;

        let x1 = point[0] + 26519.0 / 65536.0;
        let y1 = point[1] + 18128.0 / 65536.0;
        let z1 = point[2] + 60943.0 / 65536.0;
        let u1 = point[3] + 48513.0 / 65536.0;

        let x2 = point[0] + 53820.0 / 65536.0;
        let y2 = point[1] + 11213.0 / 65536.0;
        let z2 = point[2] + 44845.0 / 65536.0;
        let u2 = point[3] + 39357.0 / 65536.0;

        let x3 = point[0] + 18128.0 / 65536.0;
        let y3 = point[1] + 44845.0 / 65536.0;
        let z3 = point[2] + 12414.0 / 65536.0;
        let u3 = point[3] + 60943.0 / 65536.0;

        let x_distort = point[0] + (self.distorters[0].get([x0, y0, z0, u0]) * self.power);
        let y_distort = point[1] + (self.distorters[1].get([x1, y1, z1, u1]) * self.power);
        let z_distort = point[2] + (self.distorters[2].get([x2, y2, z2, u2]) * self.power);
        let u_distort = point[3] + (self.distorters[3].get([x3, y3, z3, u3]) * self.power);

        self.source
            .get([x_distort, y_distort, z_distort, u_distort])
    }
}
