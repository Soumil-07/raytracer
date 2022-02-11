use rand::{prelude::SmallRng, SeedableRng};

use crate::ray::{random_in_unit_sphere, Color, Ray};

use super::Material;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray: &crate::ray::Ray,
        rec: &crate::ray::hittable::HitInfo,
    ) -> (bool, Ray, Color) {
        let mut rng = SmallRng::from_entropy();
        let mut scatter_direction = rec.normal + random_in_unit_sphere(&mut rng).unit();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.point, scatter_direction);
        (true, scattered, self.albedo)
    }
}
