use super::{HitInfo, Hittable, Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> (bool, HitInfo) {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        // h = b/2
        let h = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = h.powi(2) - a * c;
        if discriminant < 0.0 {
            return (false, HitInfo::empty());
        };
        let sqrtd = discriminant.sqrt();

        // find the root in the range (t_min, t_max)
        let mut root = (-h - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-h + sqrtd) / a;
            if root < t_min || root > t_max {
                return (false, HitInfo::empty());
            };
        };

        let t = root;
        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;
        let mut hit_info = HitInfo::new(point, normal, t);
        hit_info.set_face_normal(&ray, &normal);

        return (true, hit_info);
    }
}
