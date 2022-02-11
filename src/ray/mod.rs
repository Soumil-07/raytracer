pub mod hittable;
pub mod utils;
mod vec3;

pub use vec3::{random_in_hemisphere, random_in_unit_sphere, Color, Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + (self.direction * t)
    }
}
