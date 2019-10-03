use crate::prelude::*;
use crate::vector;
use super::Material;


pub struct Dielectric {
    ref_idx: f64
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, hit_rec: HitRecord) -> Option<(Vec3, Ray)> {
        let (outward_normal, ni_over_nt, cos) = if ray.direction().dot(hit_rec.normal) > 0.0 {
            (
                -hit_rec.normal,
                self.ref_idx,
                self.ref_idx * ray.direction().dot(hit_rec.normal) / ray.direction().length()
            )
        } else {
            (
                hit_rec.normal,
                1.0 / self.ref_idx,
                -ray.direction().dot(hit_rec.normal) / ray.direction().length()
            )
        };



        Some(if let Some(refracted) = vector::refract(ray.direction(), outward_normal, ni_over_nt) {
            (vec3(1.0, 1.0, 1.0), Ray::new(hit_rec.point, refracted))
        } else {
            let reflected = vector::reflect(ray.direction(), hit_rec.normal);
            (vec3(1.0, 1.0, 1.0), Ray::new(hit_rec.point, reflected))
        })
    }
}
//
//fn schlick(cos: f64, ref_idx: f64) -> f64 {
//    let r0 =(1.0-ref_idx) / (1.0+ref_idx);
//    let r0 = r0 * r0;
//    r0 + (1.0-r0) *
//}