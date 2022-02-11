use std::rc::Rc;

use crate::material::Material;

use super::{HitInfo, Hittable, Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64, hit_info: &mut HitInfo) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        // h = b/2
        let h = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = h.powi(2) - a * c;
        if discriminant < 0.0 {
            return false;
        };
        let sqrtd = discriminant.sqrt();

        // find the root in the range (t_min, t_max)
        let mut root = (-h - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-h + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            };
        };

        hit_info.t = root;
        hit_info.point = ray.at(hit_info.t);
        hit_info.normal = (hit_info.point - self.center) / self.radius;
        hit_info.set_face_normal(&ray, &hit_info.normal.clone());
        hit_info.material = Some(Rc::clone(&self.material));

        return true;
    }
}
