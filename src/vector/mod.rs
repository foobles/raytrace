pub mod ray;
pub mod vec3;

pub use ray::Ray;
pub use vec3::Vec3;

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * v.dot(n)*2.0 // ???
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let disc = 1.0 - ni_over_nt * ni_over_nt * (1.0-dt*dt);
    if disc > 0.0 {
        Some((uv - n*dt)*ni_over_nt - n * disc.sqrt())
    } else {
        None
    }
}