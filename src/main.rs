mod vector;
pub mod hit;
pub mod mat;
pub mod prelude;

use std::fs::File;
use std::io::Write;
use std::thread;
use rand::Rng;

use prelude::*;

use hit::{Hittable, Sphere, HittableList};
use mat::{Lambertian, Metal, Dielectric};

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
            horizontal: vec3(width, 0.0, 0.0),
            vertical: vec3(0.0, height, 0.0)
        }
    }

    fn get_ray(self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + self.horizontal*u + self.vertical*v - self.origin
        )
    }
}



fn color(ray: Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    const MAX_DEPTH: i32 = 50;
    if let Some(rec) = world.hit(ray, 0.001, std::f64::MAX) {
        if let Some((attenuation, scatter)) = rec.material.scatter(ray, rec) {
            if depth < MAX_DEPTH {
                return attenuation * color(scatter, world, depth + 1);
            }
        }
        vec3(0.0, 0.0, 0.0)
    } else {
        let dir = ray.direction().normalize();
        let t = (dir.x() + 1.0) / 2.0;
        vec3(1.0, 1.0, 1.0) * (1.0 - t) + vec3(0.5, 0.7, 1.0) * t
    }
}

fn render_to_file(file: &mut File, world: &dyn Hittable, camera: &Camera) -> Result<(), std::io::Error> {
    const NX: i32 = 400;
    const NY: i32 = 200;
    const NS: i32 = 100;

    writeln!(file, "P3\n{} {}\n255", NX, NY)?;
    let mut rng = rand::thread_rng();
    let mut progress = 0.0;
    let mut prev_progress = 0.0;
    for j in (0..NY).rev() {
        for i in 0..NX {
            let mut c = Vec3::zero();
            for _ in 0..NS {
                let u = (i as f64 + rng.gen::<f64>()) / NX as f64;
                let v = (j as f64 + rng.gen::<f64>()) / NY as f64;
                let r = camera.get_ray(u, v);
                c += color(r, world, 0);
            }
            c /= NS as f64;
            c = vec3(c.r().sqrt(), c.g().sqrt(), c.b().sqrt());

            let ir = (c.r() * 255.99) as i32;
            let ig = (c.g() * 255.99) as i32;
            let ib = (c.b() * 255.99) as i32;
            writeln!(file, "{} {} {}", ir, ig, ib)?;

            progress += 1.0 / (NY * NX) as f64 * 100.0;
            if progress >= prev_progress + 1.0 {
                println!("{:.2}%", progress);
                prev_progress = progress;
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {

    let camera = Camera::new(
        vec3(0.0, 0.0, 2.0),
        vec3(-2.0, -1.0, -1.0),
        4.0,
        2.0
    );

    let mut handles = Vec::with_capacity(40);

    for i in 0..40 {

        let handle = thread::spawn(move || {
            let out_file_name = format!("out/out{}.ppm", i);
            let mut out_file = File::create(&out_file_name).unwrap();

            let x =  -0.5 + 2.5 * i as f64 / 40.0;

            let world = HittableList::new(vec![
                Box::new(Sphere::new(
                    vec3(0.0, 0.6 - i as f64 * -0.6 / 40.0, -1.8),
                    0.6,
                    Box::new(Lambertian::new(vec3(0.8, 0.3, 0.3)))
                )),
                Box::new(Sphere::new(
                    vec3(0.0, -100.5, -1.0),
                    100.0,
                    Box::new(Lambertian::new(vec3(0.3 + i as f64 * 0.5/40.0, 0.8 - i as f64 * 0.5 / 40.0, i as f64 * 0.2 / 40.0)))
                )),
                Box::new(Sphere::new(
                    vec3(1.5, 0.0, -1.0),
                    0.2 + 0.7 * i as f64 / 40.0 ,
                    Box::new(Metal::new(vec3(0.8, 0.6, 0.2), 0.0))
                )),
                Box::new(Sphere::new(
                    vec3(x, 0.0, -0.6),
                    -0.45/2.0,
                    Box::new(Dielectric::new(1.45))
                )),
                Box::new(Sphere::new(
                    vec3(x, 0.0, -0.6),
                    0.25,
                    Box::new(Dielectric::new(1.45))
                ))
            ]);
            render_to_file(&mut out_file, &world, &camera).unwrap();
        });

        handles.push(handle);

    }

    for h in handles {
        h.join().unwrap();
    }

    Ok(())
}
