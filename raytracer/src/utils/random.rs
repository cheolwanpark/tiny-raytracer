use std::ops::Range;

use rand::{distributions::uniform::SampleUniform, Rng};

use crate::{Float, Int};

pub fn random_int<T: SampleUniform + PartialOrd + From<u32>>() -> T {
    random_range(T::from(0)..T::from(1))
}

pub fn random<T: SampleUniform + PartialOrd + From<f32>>() -> T {
    random_range(T::from(0.0)..T::from(1.0))
}

pub fn random_range<T: SampleUniform + PartialOrd>(range: Range<T>) -> T {
    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}