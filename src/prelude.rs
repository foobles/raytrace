pub use crate::vector::{Ray, Vec3};
pub use crate::hit::{self, HitRecord};
pub use crate::mat;

use rand::prelude::*;

pub fn random_in_unit_sphere() -> Vec3 {
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