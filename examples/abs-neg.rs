extern crate noise;

use noise::{utils::*, Abs, Negate, Perlin};

mod utils;

fn main() {
    let perlin1 = Perlin::default();
    let perlin2 = Perlin::default();
    let abs1 = Abs::new(perlin1);
    let abs2 = Abs::new(perlin2);
    let neg = Negate::new(abs2);

    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(abs1).build(),
        "abs_neg_abs.png",
    );
    utils::write_example_to_file(
        &PlaneMapBuilder::<_, 2>::new(neg).build(),
        "abs_neg_neg.png",
    );
}
