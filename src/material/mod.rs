use crate::ray::{hittable::HitInfo, Color, Ray};

pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitInfo) -> (bool, Ray, Color);
}
