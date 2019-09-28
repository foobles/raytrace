use crate::vector::Ray;
use super::{Hittable, HitRecord};


pub struct HittableList {
    data: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> Self {
        HittableList { data: list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, min_t: f64, max_t: f64) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        for h in &self.data {
            let t = closest_hit.map(|x| x.time).unwrap_or(max_t);
            if let Some(hit_rec) = h.hit(ray, min_t, t) {
                closest_hit = Some(hit_rec);
            }
        }
        closest_hit
    }
}