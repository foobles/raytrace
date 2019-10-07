use crate::prelude::*;
use hit::Hittable;
use std::ops::{Deref, DerefMut};


pub struct HittableList {
    data: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> Self {
        HittableList { data: list }
    }
}

impl Deref for HittableList {
    type Target = Vec<Box<dyn Hittable>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for HittableList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
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