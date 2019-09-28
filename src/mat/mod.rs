use crate::vector::{Ray, Vec3};
use crate::hit::{HitRecord};

mod lambertian;
pub use lambertian::Lambertian;

pub trait Material {
    fn scatter(&self, ray: Ray, hit_rec: HitRecord) -> Option<(Vec3, Ray)>;
}



