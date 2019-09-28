use super::Material;
use crate::vector::{Vec3, Ray};
use crate::hit::HitRecord;

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

fn random_in_unit_sphere() -> Vec3 {
    //let mut rng = thread_rng();
    loop {
        let p = Vec3::new(random(), random(), random()) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        } else {
            continue;
        }
    }
}