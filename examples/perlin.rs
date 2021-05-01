//! An example of using perlin noise

extern crate noise;

use noise::{utils::*, Perlin, Seedable};

fn main() {
    let perlin = Perlin::default();

    PlaneMapBuilder::new(&perlin)
        .with_size(1024, 1024)
        .with_x_bounds(-5.0, 5.0)
        .with_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("perlin.png");

    let perlin = perlin.with_seed(1);

    PlaneMapBuilder::new(&perlin)
        .with_size(1024, 1024)
        .with_x_bounds(-5.0, 5.0)
        .with_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("perlin_seed=1.png");
}
