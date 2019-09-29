use super::Material;
use crate::prelude::*;

pub struct Metal {
    albedo: Vec3,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(ray.direction().normalize(), rec.normal);
        let scatter = Ray::new(rec.point, reflected + random_in_unit_sphere() * self.fuzz);
        if scatter.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scatter))
        } else {
            None
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * v.dot(n)*2.0 // ???
}