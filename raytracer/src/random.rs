use std::ops::Range;

use rand::Rng;

use crate::Float;

pub fn random_float_range(range: Range<Float>) -> Float {
    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}

pub fn random_float() -> Float {
    random_float_range(0.0..1.0)
}
