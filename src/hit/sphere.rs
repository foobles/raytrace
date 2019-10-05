use super::{Material, Hittable};
use crate::prelude::*;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Self {
        Sphere { center, radius, material }
    }

    pub fn material(&self) -> &dyn Material {
        self.material.as_ref()
    }

    pub fn material_mut(&mut self) -> &mut dyn Material {
        self.material.as_mut()
    }

    pub fn set_material(&mut self, material: Box<dyn Material>) {
        self.material = material
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn set_center(&mut self, center: Vec3) {
        self.center = center
    }


    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius
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