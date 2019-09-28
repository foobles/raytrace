mod vector;
mod hit;

use std::fs::File;
use std::io::Write;
use vector::{Ray, Vec3};
use hit::{Hittable, HitRecord, Sphere, HittableList};

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
    let nx = 400;
    let ny = 200;
    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    writeln!(&mut out_file, "P3\n{} {}\n255", nx, ny)?;

    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0))
    ]);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = Ray::new(origin, lower_left + horizontal*u + vertical*v);
            let c = color(r, &world);

            let ir = (c.r() * 255.99) as i32;
            let ig = (c.g() * 255.99) as i32;
            let ib = (c.b() * 255.99) as i32;

            writeln!(&mut out_file, "{} {} {}", ir, ig, ib)?;
        }
    }
    Ok(())
}
