use crate::{math::vec3::Vec3, ray::Ray, Float};

pub struct Camera {
    position: Vec3,
    viewport_upper_left: Vec3,
    forward: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        focal_length: Float,
        position: Vec3,
        look_at: Vec3,
        up: Vec3,
        vertical_fov: Float,
        aspect_ratio: Float,
    ) -> Camera {
        let viewport_height = 2.0 * focal_length * (vertical_fov.to_radians() / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_at - position).normalized();
        let u = up.cross(&w).normalized();
        let v = w.cross(&u).normalized();

        let forward = w * focal_length;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let viewport_upper_left = position - horizontal / 2.0 + vertical / 2.0
            + forward;

        Camera {
            position,
            viewport_upper_left,
            forward,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: Float, v: Float) -> Ray {
        Ray::new(
            self.position,
            self.viewport_upper_left + (u * self.horizontal) - (v * self.vertical)
                - self.position,
        )
    }
}

#[cfg(test)]
mod test {
    use crate::utils::image::Image;

    use super::*;
    #[test]
    fn test_new() {
        let focal_length = 1.0;
        let position = Vec3::new(0.0, 0.0, 0.0);
        let look_at = Vec3::new(0.0, 0.0, 1.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let vertical_fov = 90.0;
        let aspect_ratio = 16.0 / 9.0;

        let camera = Camera::new(
            focal_length,
            position,
            look_at,
            up,
            vertical_fov,
            aspect_ratio,
        );

        assert_eq!(camera.position, position);
        assert_eq!(camera.viewport_upper_left, Vec3::new(-aspect_ratio, 1.0, 1.0));
        assert_eq!(camera.forward, Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(camera.horizontal, Vec3::new(2.0 * aspect_ratio, 0.0, 0.0));
        assert_eq!(camera.vertical, Vec3::new(0.0, 2.0, 0.0));
    }

    #[test]
    fn test_get_ray() {
        let focal_length = 1.0;
        let position = Vec3::new(0.0, 0.0, 0.0);
        let look_at = Vec3::new(0.0, 0.0, 1.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let vertical_fov = 90.0;
        let aspect_ratio = 16.0 / 9.0;

        let camera = Camera::new(
            focal_length,
            position,
            look_at,
            up,
            vertical_fov,
            aspect_ratio,
        );

        assert_eq!(camera.get_ray(0.5, 0.5).direction(), camera.forward.normalized());
        assert_eq!(camera.get_ray(0.0, 0.0).direction(), camera.viewport_upper_left.normalized());
        assert_eq!(camera.get_ray(1.0, 1.0).direction(), Vec3::new(-camera.viewport_upper_left.x, -camera.viewport_upper_left.y, focal_length).normalized());
    }

    #[test]
    #[ignore]
    fn test_ray_image_generation() {
        let mut image = Image::new(800, 450);
        let camera = Camera::new(
            1.0,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            image.aspect_ratio()
        );
        let (width, height) = image.size();
        for j in 0..height {
            for i in 0..width {
                let ray = camera.get_ray(i as Float / width as Float, j as Float / height as Float);
                let a = 0.5 * ray.direction().y + 1.0;
                let col = (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0);
                image.set_pixel(i, j, col.into());
            }
        }
        image.save("output/test_ray_image_generation.png")
    }
}