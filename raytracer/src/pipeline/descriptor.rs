use crate::camera::Camera;

pub(super) struct ImageDescriptor {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: usize,
}