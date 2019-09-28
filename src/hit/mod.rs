mod sphere;
mod list;

use crate::vector::{Ray, Vec3};

pub use sphere::Sphere;
pub use list::HittableList;

pub trait Hittable {
    fn hit(&self, ray: Ray, min_t: f64, max_t: f64) -> Option<HitRecord>;
}

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub time: f64,
    pub point: Vec3,
    pub normal: Vec3
}