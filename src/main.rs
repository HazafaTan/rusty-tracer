use std::fs::File;
use std::io::prelude::Write;
mod vec3;
use vec3::Vec3;

fn main() -> std::io::Result<()> {
    let image_width: u16 = 256;
    let image_height: u16 = 256;
    let mut file = File::create("out.ppm")?;
    file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;
    for i in (0..image_height).rev() {
        for j in 0..image_width {
            let r: f32 = (i as f32) / ((image_width - 1) as f32);
            let g: f32 = (j as f32) / ((image_height - 1) as f32);
            let b: f32 = 0.25;
            let ir: u8 = (r * 255.99) as u8;
            let ig: u8 = (g * 255.99) as u8;
            let ib: u8 = (b * 255.99) as u8;
            file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }
    let meow = Vec3::new(1.0, 2.0, 3.0);
    println!("{}", Vec3::length(meow));
    Ok(())
}
