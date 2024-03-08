use crate::{math::vec3::Vec3, Float};

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: Float) -> Vec3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_at() {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(1.0, 2.0, -1.0);
        let ray = Ray::new(origin, direction);
        
        assert_eq!(ray.at(2.5), Vec3::new(2.5, 5.0, -2.5));
    }
}

