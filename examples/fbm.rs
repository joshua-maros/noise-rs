//! An example of using the fBm noise function

extern crate noise;

use noise::{utils::*, Fbm};

fn main() {
    let fbm = Fbm::new();

    PlaneMapBuilder::new(&fbm)
        .with_size(1000, 1000)
        .with_x_bounds(-5.0, 5.0)
        .with_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm.png");
}
