use super::{HitInfo, Hittable};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64, hit_info: &mut HitInfo) -> bool {
        let mut hit_anything = false;
        let mut closest_hit = t_max;
        let mut rec = HitInfo::empty();

        for object in &self.objects {
            if object.hit(ray, t_min, closest_hit, &mut rec) {
                hit_anything = true;
                closest_hit = rec.t;
                *hit_info = rec.clone();
            }
        }

        hit_anything
    }
}
