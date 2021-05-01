extern crate noise;

use noise::{utils::*, Checkerboard, ScalePoint};

fn main() {
    let cboard = Checkerboard::default();
    let scale_point = ScalePoint::new(cboard).with_all_scales(1.0, 2.0, 3.0, 1.0);

    PlaneMapBuilder::new(&scale_point)
        .with_size(500, 500)
        .with_x_bounds(-5.0, 5.0)
        .with_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("scale_point.png");
}
