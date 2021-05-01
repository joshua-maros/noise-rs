extern crate noise;

use noise::{utils::*, Clamp, Perlin};

fn main() {
    let perlin = Perlin::default();
    let clamp = Clamp::new(&perlin)
        .with_lower_bound(0.0)
        .with_upper_bound(0.5);

    PlaneMapBuilder::new(&clamp)
        .build()
        .write_to_file("clamp.png");
}
