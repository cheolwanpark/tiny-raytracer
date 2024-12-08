use crate::{math::vec3::Vec3, ray::Ray, Float};

#[derive(Clone)]
pub struct Camera {
    position: Vec3,
    viewport_upper_left: Vec3,
    forward: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    width: usize,
    height: usize,
}

impl Camera {
    pub fn new(
        focus_distance: Float,
        defocus_angle: Float,
        position: Vec3,
        look_at: Vec3,
        up: Vec3,
        vertical_fov: Float,
        width: usize,
        height: usize,
    ) -> Camera {
        let viewport_height = 2.0 * focus_distance * (vertical_fov.to_radians() / 2.0).tan();
        let aspect_ratio = width as Float / height as Float;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (position - look_at).normalized();
        let u = up.cross(&w).normalized();
        let v = w.cross(&u).normalized();

        let forward = w * focus_distance;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let viewport_upper_left = position - horizontal / 2.0 + vertical / 2.0
            - forward;

        let defocus_radius = focus_distance * (defocus_angle.to_radians() / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            position,
            viewport_upper_left,
            forward,
            horizontal,
            vertical,
            defocus_disk_u,
            defocus_disk_v,
            width,
            height,
        }
    }

    pub fn get_ray(&self, u: Float, v: Float) -> Ray {
        let p = Vec3::new_random_in_unit_disk();
        let origin = self.position + p[0]*self.defocus_disk_u + p[1]*self.defocus_disk_v;
        Ray::new(
            origin,
            self.viewport_upper_left + (u * self.horizontal) - (v * self.vertical)
                - origin,
        )
    }

    pub fn get_image_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

#[cfg(test)]
mod test {
    use crate::utils::image::Image;

    use super::*;
    #[test]
    fn test_new() {
        let focal_length = 1.0;
        let defocus_angle = 10.0;
        let position = Vec3::new(0.0, 0.0, 0.0);
        let look_at = Vec3::new(0.0, 0.0, 1.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let vertical_fov = 90.0;
        let width = 16;
        let height = 9;
        let aspect_ratio = width as Float / height as Float;

        let camera = Camera::new(
            focal_length,
            defocus_angle,
            position,
            look_at,
            up,
            vertical_fov,
            width, height
        );

        assert_eq!(camera.position, position);
        assert_eq!(camera.viewport_upper_left, Vec3::new(aspect_ratio, 1.0, 1.0));
        assert_eq!(camera.forward, Vec3::new(0.0, 0.0, -1.0));
        assert_eq!(camera.horizontal, Vec3::new(-2.0 * aspect_ratio, 0.0, 0.0));
        assert_eq!(camera.vertical, Vec3::new(0.0, 2.0, 0.0));
    }

    #[test]
    #[ignore]
    fn test_ray_image_generation() {
        let mut image = Image::new(800, 450);
        let (width, height) = image.size();
        let camera = Camera::new(
            1.0,
            10.0,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            width, height
        );
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