use std::ops::Mul;

use ndarray::{arr2, Array2};

use crate::Float;

use super::vec3::Vec3;

#[derive(Clone)]
pub struct Transform {
    mat: Array2<Float>
}

impl Transform {
    pub fn new(
        translation: Vec3,
        rotation: Vec3,
        scale: Vec3
    ) -> Self {
        Self::translate(translation) * Self::scale(scale) * Self::rotate(rotation)
    }

    pub fn identity() -> Self {
        let mat = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        Self { mat }
    }

    pub fn translate(delta: Vec3) -> Self {
        let mat = arr2(&[
            [1.0, 0.0, 0.0, delta.x],
            [0.0, 1.0, 0.0, delta.y],
            [0.0, 0.0, 1.0, delta.z],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        Self { mat }
    }

    pub fn rotate_x(angle: Float) -> Self {
        let (cos, sin) = (angle.cos(), angle.sin());
        let mat = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos, -sin, 0.0],
            [0.0, sin, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        Self { mat }
    }

    pub fn rotate_y(angle: Float) -> Self {
        let (cos, sin) = (angle.cos(), angle.sin());
        let mat = arr2(&[
            [cos, 0.0, sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        Self { mat }
    }

    pub fn rotate_z(angle: Float) -> Self {
        let (cos, sin) = (angle.cos(), angle.sin());
        let mat = arr2(&[
            [cos, -sin, 0.0, 0.0],
            [sin, cos, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        Self { mat }
    }

    pub fn rotate(angles: Vec3) -> Self {
        Self::rotate_z(angles.z) * Self::rotate_y(angles.y) * Self::rotate_x(angles.x)
    }

    pub fn scale(scales: Vec3) -> Self {
        let mat = arr2(&[
            [scales.x, 0.0, 0.0, 0.0],
            [0.0, scales.y, 0.0, 0.0],
            [0.0, 0.0, scales.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        Self { mat }
    }
}

impl Mul<Transform> for Transform {
    type Output = Self;
    fn mul(self, rhs: Transform) -> Self::Output {
        let mat = self.mat.dot(&rhs.mat);
        Self { mat }
    }
}

impl Mul<Vec3> for Transform {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        let v = arr2(&[
            [rhs.x], 
            [rhs.y], 
            [rhs.z], 
            [1.0],
        ]);
        let v = self.mat.dot(&v);
        Self::Output::new(v[[0, 0]], v[[1, 0]], v[[2, 0]])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation() {
        let v1 = Vec3::zero();
        let v2 = Vec3::new(10.0, 15.0, 20.0);
        let delta = Vec3::new(1.0, -2.0, 3.0);
        let translation = Transform::translate(Vec3::new(1.0, -2.0, 3.0));
        assert_eq!(v1+delta, translation.clone()*v1);
        assert_eq!(v2+delta, translation*v2);
    }

    #[test]
    fn test_scale() {
        let v1 = Vec3::zero();
        let v2 = Vec3::new(10.0, 15.0, 20.0);
        let scale = Vec3::new(1.0, -2.0, 3.0);
        let scaling = Transform::scale(scale);
        assert_eq!(v1*scale, scaling.clone()*v1);
        assert_eq!(v2*scale, scaling*v2);
    }
    
    #[test]
    fn test_rotation() {
        let zero = Vec3::zero();
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 1.0);
        let rot1 = Transform::rotate_x(90_f32.to_radians());
        let rot2 = Transform::rotate_y(90_f32.to_radians());
        let rot3 = Transform::rotate_z(90_f32.to_radians());

        assert_eq!(zero, rot1.clone()*zero);
        assert_eq!(v1, rot1.clone()*v1);
        assert_eq!(Vec3::new(0.0, 0.0, -1.0), rot2.clone()*v1);
        assert_eq!(Vec3::new(0.0, 1.0, 0.0), rot3.clone()*v1);
        assert_eq!(Vec3::new(0.0, -1.0, 1.0), rot1*v2);
        assert_eq!(Vec3::new(1.0, 1.0, 0.0), rot2*v2);
        assert_eq!(Vec3::new(-1.0, 0.0, 1.0), rot3*v2);
    }

    #[test]
    fn complex_transform() {
        let zero = Vec3::zero();
        let v = Vec3::new(1.0, 1.0, 0.0);
        let t = Transform::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(90_f32.to_radians(), -90_f32.to_radians(), -90_f32.to_radians()),
            Vec3::new(2.0, 3.0, 4.0),
        );
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), t.clone()*zero);
        assert_eq!(Vec3::new(1.0, 5.0, 7.0), t*v);
    }
}