mod vector;
mod hit;

use std::fs::File;
use std::io::Write;
use vector::{Ray, Vec3};
use hit::{Hittable, HitRecord, Sphere, HittableList};
use rand::Rng;

#[derive(Clone, Copy, Debug)]
struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    fn new(origin: Vec3, lower_left: Vec3, width: f64, height: f64) -> Self {
        Camera {
            origin,
            lower_left,
            horizontal: Vec3::new(width, 0.0, 0.0),
            vertical: Vec3::new(0.0, height, 0.0)
        }
    }

    fn get_ray(self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + self.horizontal*u + self.vertical*v - self.origin
        )
    }
}

fn color(ray: Ray, world: &dyn Hittable) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.0, std::f64::MAX) {
        Vec3::new(rec.normal.x() + 1.0, rec.normal.y() + 1.0, rec.normal.z() + 1.0) / 2.0
    } else {
        let dir = ray.direction().normalize();
        let t = (dir.y() + 1.0) / 2.0;
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut out_file = File::create("out/out.ppm")?;
    const NX: i32 = 200;
    const NY: i32 = 100;
    const NS: i32 = 100;
    writeln!(&mut out_file, "P3\n{} {}\n255", NX, NY)?;

    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0))
    ]);

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
        4.0,
        2.0
    );
    let mut rng = rand::thread_rng();
    for j in (0..NY).rev() {
        for i in 0..NX {
            let mut c = Vec3::empty();
            for _ in 0..NS {
                let u = (i as f64 + rng.gen::<f64>()) / NX as f64;
                let v = (j as f64 + rng.gen::<f64>()) / NY as f64;
                let r = camera.get_ray(u, v);
                c += color(r, &world);
            }
            c /= NS as f64;
            let ir = (c.r() * 255.99) as i32;
            let ig = (c.g() * 255.99) as i32;
            let ib = (c.b() * 255.99) as i32;

            writeln!(&mut out_file, "{} {} {}", ir, ig, ib)?;
        }
    }
    Ok(())
}
