use super::Material;
use crate::vector::{Vec3, Ray};
use crate::hit::HitRecord;
use crate::random_in_unit_sphere;

use rand::prelude::*;

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Vec3, Ray)> {
        let target_dir = rec.normal + random_in_unit_sphere();
        Some((self.albedo, Ray::new(rec.point, target_dir)))
    }
}