pub type Float = f32;
pub use std::f32::consts as FloatConsts;

pub mod image;
pub mod math;
pub mod ray;
pub mod camera;
pub mod hittable;
pub mod renderer;
pub mod random;