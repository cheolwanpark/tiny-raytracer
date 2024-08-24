#![allow(dead_code)]
#![allow(unused_imports)]

pub type Float = f32;
pub use std::f32::consts as FloatConsts;

pub mod camera;
pub mod hittable;
pub mod image;
pub mod material;
pub mod math;
pub mod random;
pub mod ray;
pub mod renderer;

pub mod accel;
