extern crate noise;

use noise::{utils::*, *};

fn main() {
    // Base wood texture. Uses concentric cylinders aligned on the z-axis, like a log.
    let base_wood = Cylinders::new().with_frequency(16.0);

    // Basic Multifractal noise to use for the wood grain.
    let wood_grain_noise = BasicMulti::new()
        .with_seed(0)
        .with_frequency(48.0)
        .with_persistence(0.5)
        .with_lacunarity(2.20703125)
        .with_octaves(3);

    // Stretch the perlin noise in the same direction as the center of the log. Should
    // produce a nice wood-grain texture.
    let scaled_base_wood_grain = ScalePoint::new(wood_grain_noise).with_z_scale(0.25);

    // Scale the wood-grain values so that they can be added to the base wood texture.
    let wood_grain = ScaleBias::new(&scaled_base_wood_grain)
        .with_scale(0.25)
        .with_bias(0.125);

    // Add the wood grain texture to the base wood texture.
    let combined_wood = Add::new(&base_wood, &wood_grain);

    // Slightly perturb the wood to create a more realistic texture.
    let perturbed_wood = Turbulence::new(combined_wood)
        .with_seed(1)
        .with_frequency(4.0)
        .with_power(1.0 / 256.0)
        .with_roughness(4);

    // Cut the wood texture a small distance from the center of the log.
    let translated_wood = TranslatePoint::new(perturbed_wood).with_y_translation(1.48);

    // Set the cut on a angle to produce a more interesting texture.
    let rotated_wood = RotatePoint::new(translated_wood).with_angles(84.0, 0.0, 0.0, 0.0);

    // Finally, perturb the wood texture again to produce the final texture.
    let final_wood = Turbulence::new(rotated_wood)
        .with_seed(2)
        .with_frequency(2.0)
        .with_power(1.0 / 64.0)
        .with_roughness(4);

    let planar_texture = PlaneMapBuilder::new(&final_wood)
        .with_size(1024, 1024)
        .build();

    // Create a wood palette.
    let wood_gradient = ColorGradient::new()
        .clear_gradient()
        .add_gradient_point(-1.000, [189, 94, 4, 255])
        .add_gradient_point(0.500, [144, 48, 6, 255])
        .add_gradient_point(1.0, [60, 10, 8, 255]);

    let mut renderer = ImageRenderer::new().with_gradient(wood_gradient);

    renderer
        .render(&planar_texture)
        .write_to_file("texture_wood_planar.png");
}
