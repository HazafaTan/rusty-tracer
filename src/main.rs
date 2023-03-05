use std::fs::File;
use std::io::prelude::Write;
mod vec3;
use vec3::{color, point3, Vec3};
mod ray;
use ray::Ray;

fn main() -> std::io::Result<()> {
    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;

    let image_width: u16 = 400;
    let image_height: u16 = (image_width as f64 / aspect_ratio) as u16;

    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = Vec3::sub(
        Vec3::sub(
            Vec3::sub(origin, Vec3::divide(horizontal, 2.0)),
            Vec3::divide(vertical, 2.0),
        ),
        Vec3 {
            x: (0.0),
            y: (0.0),
            z: (focal_length),
        },
    );

    let mut file = File::create("out.ppm")?;
    file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;
    for i in (0..image_height).rev() {
        for j in 0..image_width {
            let u: f64 = (j as f64) / ((image_width - 1) as f64);
            let v: f64 = (i as f64) / ((image_height - 1) as f64);
            let r = Ray::new(
                Vec3::add(
                    Vec3::add(Vec3::multiply(horizontal, u), lower_left_corner),
                    Vec3::sub(Vec3::multiply(vertical, v), origin),
                ),
                origin,
            );
            let pixel_color = ray_color(r);
            write_color(&mut file, pixel_color)?;
        }
    }
    Ok(())
}

fn write_color<W: Write>(out: &mut W, pixel_color: color) -> std::io::Result<()> {
    write!(
        out,
        "{} {} {}\n",
        (255.999 * pixel_color.x) as i32,
        (255.999 * pixel_color.y) as i32,
        (255.999 * pixel_color.z) as i32
    )
}

fn ray_color(r: Ray) -> Vec3 {
    let t = hit_sphere(point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let N = Vec3::unit_vector(Vec3::sub(
            Ray::at(t, r),
            Vec3 {
                x: (0.0),
                y: (0.0),
                z: (-1.0),
            },
        ));
        return Vec3::multiply(color::new(N.x + 1.0, N.y + 1.0, N.z + 1.0), 0.5);
    }
    let unit_direction = Vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    return Vec3::add(
        Vec3::multiply(color::new(0.5, 0.7, 1.), t),
        Vec3::multiply(color::new(1.0, 1.0, 1.0), 1.0 - t),
    );
}

fn hit_sphere(center: point3, radius: f64, r: ray::Ray) -> f64 {
    let oc = Vec3::sub(r.origin, center);
    let a = Vec3::square_length(r.direction);
    let half_b = Vec3::dot(oc, r.direction);
    let c = Vec3::square_length(oc) - radius * radius;
    let discriminant = (half_b * half_b) - (a * c);
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return ((-half_b) - discriminant.sqrt()) / a;
    }
}
