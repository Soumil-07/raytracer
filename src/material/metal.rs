use rand::{prelude::SmallRng, SeedableRng};

use crate::ray::{random_in_unit_sphere, utils::reflect, Color, Ray};

use super::Material;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &crate::ray::Ray,
        rec: &crate::ray::hittable::HitInfo,
    ) -> (bool, Ray, Color) {
        let reflected = reflect(&ray.direction.unit(), &rec.normal);
        let mut rng = SmallRng::from_entropy();
        let scattered = Ray::new(
            rec.point,
            reflected + (random_in_unit_sphere(&mut rng) * self.fuzz),
        );

        (
            scattered.direction.dot(&rec.normal) > 0.0,
            scattered,
            self.albedo,
        )
    }
}
