use std::fs::File;
use std::io::prelude::Write;
mod vec3;
use vec3::Vec3;
mod ray;
use ray::Ray;

fn main() -> std::io::Result<()> {
    let image_width: u16 = 256;
    let image_height: u16 = 256;
    let mut file = File::create("out.ppm")?;
    file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;
    for i in (0..image_height).rev() {
        for j in 0..image_width {
            let r: f64 = (i as f64) / ((image_width - 1) as f64);
            let g: f64 = (j as f64) / ((image_height - 1) as f64);
            let b: f64 = 0.25;
            let pixel_color = Vec3::new(r, g, b);
            write_color(&mut file, pixel_color)?;
        }
    }
    let meow = Vec3::new(1.0, 2.0, 3.0);
    println!("{}", Vec3::length(meow));
    Ok(())
}

fn write_color<W: Write>(out: &mut W, pixel_color: vec3::color) -> std::io::Result<()> {
    write!(
        out,
        "{} {} {}\n",
        (255.999 * pixel_color.x) as i32,
        (255.999 * pixel_color.y) as i32,
        (255.999 * pixel_color.z) as i32
    )
}
