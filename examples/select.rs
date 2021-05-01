extern crate noise;

use noise::{utils::*, Checkerboard, Constant, Cylinders, Perlin, Select};

fn main() {
    let checkerboard = &Checkerboard::default();
    let cylinders = &Cylinders::new();
    let perlin = &Perlin::default();
    let constant = &Constant::new(0.5);
    let select1 = Select::new(&perlin, &cylinders, &checkerboard)
        .with_bounds(0.0, 1.0)
        .with_falloff(0.5);
    let select2 = Select::new(&perlin, &constant, &checkerboard)
        .with_bounds(0.0, 1.0)
        .with_falloff(0.0);

    PlaneMapBuilder::new(&select1)
        .build()
        .write_to_file("select1.png");
    PlaneMapBuilder::new(&select2)
        .build()
        .write_to_file("select2.png");
}
