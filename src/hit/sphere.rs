use crate::vector::{Vec3, Ray};
use super::{HitRecord, Hittable};

pub struct Sphere {
    center: Vec3,
    radius: f64
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, min_t: f64, max_t: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let disc = b*b - a*c;
        if disc >= 0.0 {
            let mut time = (-b - disc.sqrt()) / a;
            if time < min_t || time > max_t {
                time = (-b + disc.sqrt()) / a;
            }
            if time < min_t || time > max_t {
                return None;
            }
            let point = ray.position_at(time);
            Some(HitRecord{
                time,
                point,
                normal: (point - self.center) / self.radius
            })
        } else {
            None
        }
    }
}