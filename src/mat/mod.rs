use crate::prelude::*;

mod lambertian;
mod metal;
mod dielectric;

pub use lambertian::Lambertian;
pub use metal::Metal;
pub use dielectric::Dielectric;

pub trait Material {
    fn scatter(&self, ray: Ray, hit_rec: HitRecord) -> Option<(Vec3, Ray)>;
}



