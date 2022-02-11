use rand::{prelude::SmallRng, Rng, SeedableRng};

use crate::ray::{
    utils::{reflect, refract},
    Color, Ray,
};

use super::Material;

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Dielectric {
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Schlick's approximation for reflectance
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &crate::ray::Ray,
        rec: &crate::ray::hittable::HitInfo,
    ) -> (bool, Ray, Color) {
        let attenuation = Color::from_coords(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray.direction.unit();
        let cos_theta = f64::min(-unit_direction.dot(&rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let mut rng = SmallRng::from_entropy();

        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
        {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        (true, Ray::new(rec.point, direction), attenuation)
    }
}
