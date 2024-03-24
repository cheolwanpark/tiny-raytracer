use rand::Rng;

use crate::Float;

pub fn random_float() -> Float {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}