#![allow(dead_code)]
#![allow(unused_imports)]

pub type Float = f32;
pub use std::f32::consts as FloatConsts;

pub mod utils;
pub mod camera;
pub mod hittable;
pub mod material;
pub mod math;
pub mod ray;
pub mod pipeline;

pub mod accel;
