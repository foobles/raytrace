mod vector;

use std::fs::File;
use std::io::Write;
use vector::{Ray, Vec3};

fn hit_sphere(ray: Ray, center: Vec3, rad: f64) -> Option<f64> {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - rad * rad;
    let disc = b*b - 4.0*a*c;
    if disc >= 0.0 {
        Some((-b - disc.sqrt()) / (2.0 * a))
    } else {
        None
    }
}

fn color(ray: Ray) -> Vec3 {
    if let Some(t) = hit_sphere(ray, Vec3::new(0.0, 0.0, -1.0), 0.5) {
        let n = (ray.position_at(t) - Vec3::new(0.0, 0.0, -1.0)).normalize();
        return Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    }
    let dir = ray.direction().normalize();
    let t = (dir.y() + 1.0) / 2.0;
    Vec3::new(1.0, 1.0, 1.0) * (1.0-t) + Vec3::new(0.5, 0.7, 1.0) * t
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
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = Ray::new(origin, lower_left + horizontal*u + vertical*v);
            let c = color(r);

            let ir = (c.r() * 255.99) as i32;
            let ig = (c.g() * 255.99) as i32;
            let ib = (c.b() * 255.99) as i32;

            writeln!(&mut out_file, "{} {} {}", ir, ig, ib)?;
        }
    }
    Ok(())
}
