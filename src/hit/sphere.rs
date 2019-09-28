use crate::vector::{Vec3, Ray};
use crate::mat::Material;
use super::{HitRecord, Hittable};
use core::borrow::Borrow;


pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Self {
        Sphere { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, min_t: f64, max_t: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let disc = b*b - a*c;
        if disc > 0.0 {
            let mut time = (-b - disc.sqrt()) / a;
            if time <= min_t || time >= max_t {
                time = (-b + disc.sqrt()) / a;
            }
            if time <= min_t || time >= max_t {
                return None;
            }
            let point = ray.position_at(time);
            Some(HitRecord{
                time,
                point,
                normal: (point - self.center) / self.radius,
                material: self.material.as_ref()
            })
        } else {
            None
        }
    }
}