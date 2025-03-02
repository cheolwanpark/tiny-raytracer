use std::{f32::INFINITY, fmt::Display, ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range, Sub, SubAssign,
}};

use rand::random;

use crate::{utils::image::Color, Float, FloatConsts};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Vec3 { x, y, z }
    }

    pub fn new_diagonal(v: Float) -> Self {
        Vec3::new(v, v, v)
    }

    pub fn zero() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn near_zero(&self) -> bool {
        let eps = 1e-7;
        self.x.abs() < eps && self.y.abs() < eps && self.z.abs() < eps
    }

    pub fn length(&self) -> Float {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> Float {
        self.dot(&self)
    }

    pub fn normalized(&self) -> Self {
        *self / self.length()
    }

    pub fn dot(&self, other: &Self) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<Float> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Float) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for Float {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl MulAssign<Float> for Vec3 {
    fn mul_assign(&mut self, rhs: Float) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Div<Float> for Vec3 {
    type Output = Self;
    fn div(self, rhs: Float) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<Float> for Vec3 {
    fn div_assign(&mut self, rhs: Float) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Index<usize> for Vec3 {
    type Output = Float;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("out of bounds index"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("out of bounds index"),
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        let (mut v1, mut v2) = (*self, *other);
        for i in 0..3 {
            if v1[i] == Float::from(INFINITY) && v1[i] == v2[i] {
                v1[i] = 0.0;
                v2[i] = 0.0;
            }
        }
        let delta = v1 - v2;
        delta.near_zero()
    }
    
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl From<Vec3> for Color {
    fn from(vec: Vec3) -> Color {
        Color::new(vec.x, vec.y, vec.z)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use std::f32::INFINITY;

    use super::*;

    #[test]
    fn arithmetic_test() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(4.0, 5.0, 6.0);
        let vec3 = Vec3::new_diagonal(INFINITY);
        let vec4 = Vec3::new(1.0, 0.0, INFINITY);

        // Neg
        assert_eq!(-vec1, Vec3::new(-1.0, -2.0, -3.0));

        // Addition
        let result = vec1 + vec2;
        assert_eq!(result, Vec3::new(5.0, 7.0, 9.0));
        assert_eq!(vec1 + vec2, vec2 + vec1);
        let result = vec1 + vec3;
        assert!(result.x.is_infinite() && result.y.is_infinite() && result.z.is_infinite());

        // Subtraction
        let result = vec1 - vec2;
        assert_eq!(result, Vec3::new(-3.0, -3.0, -3.0));
        let result = vec1 - vec3;
        assert!(result.x.is_infinite() && result.y.is_infinite() && result.z.is_infinite());

        // Multiplication
        let result = vec1 * 2.0;
        assert_eq!(result, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(vec1 * 2.0, 2.0 * vec1);
        let result = vec1 * vec4;
        assert_eq!(result, Vec3::new(1.0, 0.0, INFINITY));
        let result = vec1 * INFINITY;
        assert_eq!(result, Vec3::new_diagonal(INFINITY));

        // Division
        let result = vec1 / 2.0;
        assert_eq!(result, Vec3::new(0.5, 1.0, 1.5));
        let result = vec1 / vec4;
        assert_eq!(result, Vec3::new(1.0, INFINITY, 0.0));
        let result = vec1 / INFINITY;
        assert_eq!(result, Vec3::zero());
        let result = vec1 / 0.0;
        assert!(result.x.is_infinite() && result.y.is_infinite() && result.z.is_infinite());
    }

    #[test]
    fn vector_test() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(4.0, 5.0, 6.0);

        // Length
        let length = vec1.length();
        assert_eq!(length, (14.0 as Float).sqrt());

        // Dot product
        let dot_product = vec1.dot(&vec2);
        assert_eq!(dot_product, 32.0);

        // Cross product
        let cross_product = vec1.cross(&vec2);
        assert_eq!(cross_product, Vec3::new(-3.0, 6.0, -3.0));
    }
}
