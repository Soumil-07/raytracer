pub mod hittable_list;
pub mod sphere;

use std::rc::Rc;

use crate::material::Material;

use super::{Point3, Ray, Vec3};

pub use hittable_list::HittableList;
pub use sphere::Sphere;

#[derive(Clone)]
pub struct HitInfo {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    // Whether the normal is a front-face vector
    pub front_face: bool,
    pub material: Option<Rc<dyn Material>>,
}

impl HitInfo {
    fn set_face_normal(&mut self, ray: &Ray, normal: &Vec3) {
        self.front_face = ray.direction.dot(normal) < 0.0;
        self.normal = if self.front_face { *normal } else { -(*normal) };
    }
}

impl HitInfo {
    pub fn empty() -> Self {
        Self {
            point: Point3::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: false,
            material: None,
        }
    }

    pub fn new(point: Point3, normal: Vec3, t: f64) -> Self {
        Self {
            point,
            normal,
            t,
            front_face: false,
            material: None,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, info: &mut HitInfo) -> bool;
}
