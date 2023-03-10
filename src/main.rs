mod camera;
mod hittable;
mod ray;
mod rtweekend;
use rtweekend::{clamp, random_float};
mod sphere;
mod vec3;
use camera::Camera;
use hittable::{Hittable, HittableList};
use ray::Ray;
use sphere::Sphere;
use std::fs::File;
use std::io::prelude::Write;
use vec3::{Color, Point3, Vec3};

fn main() -> std::io::Result<()> {
    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let samples_per_pixel: u64 = 100;
    let image_width: u16 = 400;
    let image_height: u16 = (image_width as f64 / aspect_ratio) as u16;
    let max_depth = 50;

    //World
    let world = HittableList {
        objects: vec![
            Hittable::S(Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
            }),
            Hittable::S(Sphere {
                center: Vec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
            }),
        ],
    };

    // Camera
    let camera = Camera::default();

    let mut file = File::create("out.ppm")?;
    file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;
    for i in (0..image_height).rev() {
        for j in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u: f64 = (j as f64 + random_float()) / ((image_width - 1) as f64);
                let v: f64 = (i as f64 + random_float()) / ((image_height - 1) as f64);
                let r = camera.get_ray(u, v);
                pixel_color = Vec3::add(pixel_color, ray_color(r, &world, max_depth));
            }
            write_colors(&mut file, pixel_color, samples_per_pixel)?;
        }
    }
    Ok(())
}

fn write_colors<W: Write>(
    out: &mut W,
    pixel_color: Color,
    samples_per_pixel: u64,
) -> std::io::Result<()> {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    //divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    write!(
        out,
        "{} {} {}\n",
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32,
    )
}

fn ray_color(r: Ray, world: &hittable::HittableList, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        //let k = Vec3::multiply(Vec3::add(rec.normal, Color::new(1.0, 1.0, 1.0)), 0.5);
        //return k;
        let target: Point3 = Vec3::add(
            Vec3::add(rec.p, rec.normal),
            Vec3::random_in_hemisphere(rec.normal),
        );
        return Vec3::multiply(
            ray_color(
                Ray {
                    direction: (Vec3::sub(target, rec.p)),
                    origin: (rec.p),
                },
                world,
                depth - 1,
            ),
            0.5,
        );
    } else {
        let unit_direction = Vec3::unit_vector(r.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        return Vec3::add(
            Vec3::multiply(Color::new(0.5, 0.7, 1.), t),
            Vec3::multiply(Color::new(1.0, 1.0, 1.0), 1.0 - t),
        );
    }
}
