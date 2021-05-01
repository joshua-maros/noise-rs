extern crate noise;

use noise::{utils::*, *};

fn main() {
    // Primary jade texture. The ridges from the ridged-multifractal function
    // produces the veins.
    let primary_jade = RidgedMulti::new()
        .with_seed(0)
        .with_frequency(2.0)
        .with_lacunarity(2.20703125)
        .with_octaves(6);

    // Base of the secondary jade texture. The base texture uses concentric
    // cylinders aligned on the z axis, which will eventually be perturbed.
    let base_secondary_jade = Cylinders::new().with_frequency(2.0);

    // Rotate the base secondary jade texture so that the cylinders are not
    // aligned with any axis. This produces more variation in the secondary
    // jade texture since the texture is parallel to the y-axis.
    let rotated_base_secondary_jade =
        RotatePoint::new(base_secondary_jade).with_angles(90.0, 25.0, 5.0, 0.0);

    // Slightly perturb the secondary jade texture for more realism.
    let perturbed_base_secondary_jade = Turbulence::new(rotated_base_secondary_jade)
        .with_seed(1)
        .with_frequency(4.0)
        .with_power(1.0 / 4.0)
        .with_roughness(4);

    // Scale the secondary jade texture so it makes a small contribution to the
    // final jade texture.
    let secondary_jade = ScaleBias::new(&perturbed_base_secondary_jade)
        .with_scale(0.25)
        .with_bias(0.0);

    // Add the two jade textures together. These two textures were produced
    // using different combinations of coherent noise, so the final texture
    // will have a lot of variation.
    let combined_jade = Add::new(&primary_jade, &secondary_jade);

    // Finally, perturb the combined jade texture to produce the final jade
    // texture. A low roughness produces nice veins.
    let final_jade = Turbulence::new(combined_jade)
        .with_seed(2)
        .with_frequency(4.0)
        .with_power(1.0 / 16.0)
        .with_roughness(2);

    let planar_texture = PlaneMapBuilder::new(&final_jade)
        .with_size(1024, 1024)
        .build();

    let seamless_texture = PlaneMapBuilder::new(&final_jade)
        .with_size(1024, 1024)
        .with_is_seamless(true)
        .build();

    // Create a jade palette.
    let jade_gradient = ColorGradient::new()
        .clear_gradient()
        .add_gradient_point(-1.000, [24, 146, 102, 255])
        .add_gradient_point(0.000, [78, 154, 115, 255])
        .add_gradient_point(0.250, [128, 204, 165, 255])
        .add_gradient_point(0.375, [78, 154, 115, 255])
        .add_gradient_point(1.000, [29, 135, 102, 255]);

    let mut renderer = ImageRenderer::new().with_gradient(jade_gradient);

    renderer
        .render(&planar_texture)
        .write_to_file("texture_jade_planar.png");

    renderer
        .render(&seamless_texture)
        .write_to_file("texture_jade_seamless.png");
}
