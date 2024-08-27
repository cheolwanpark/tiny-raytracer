use crate::{math::vec3::Vec3, ray::Ray};

pub struct SamplePoint {
    pub x: usize,
    pub y: usize,
    pub ray: Ray,
}

pub struct SamplerInput {
    pub x: usize,
    pub y: usize,
    pub ray: Ray,
    pub remain_bounces: usize,
    pub attenuation: Vec3,
}

pub struct SampledColor {
    pub x: usize,
    pub y: usize,
    pub color: Vec3,
}