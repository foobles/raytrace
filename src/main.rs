use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out_file = File::create("out.ppm")?;
    let nx = 200;
    let ny = 100;
    writeln!(&mut out_file, "P3\n{} {}\n255", nx, ny)?;
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f64 / nx as f64;
            let g = j as f64 / ny as f64;
            let b = 0.2;
            let ir = (r * 255.99) as i32;
            let ig = (g * 255.99) as i32;
            let ib = (b * 255.99) as i32;
            writeln!(&mut out_file, "{} {} {}", ir, ig, ib)?;
        }
    }
    Ok(())
}
