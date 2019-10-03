pub mod ray;
pub mod vec3;

pub use ray::Ray;
pub use vec3::Vec3;

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * v.dot(n)*2.0 // ???
}

