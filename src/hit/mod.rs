mod sphere;
mod list;

use crate::vector::{Ray, Vec3};
use crate::mat::Material;

pub use sphere::Sphere;
pub use list::HittableList;

pub trait Hittable {
    fn hit(&self, ray: Ray, min_t: f64, max_t: f64) -> Option<HitRecord>;
}

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub time: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material
}