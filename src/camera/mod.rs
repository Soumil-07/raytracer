use crate::ray::{Point3, Ray, Vec3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;

#[derive(Debug, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let viewport_height = 2u32;
        let viewport_width = viewport_height as f64 * ASPECT_RATIO;
        let focal_length = 1.0;

        let origin = Point3::new();
        let horizontal = Vec3::from_coords(viewport_width, 0.0, 0.0);
        let vertical = Vec3::from_coords(0.0, viewport_height.into(), 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from_coords(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            vertical,
            horizontal,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        );
    }
}
