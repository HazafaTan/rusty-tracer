mod camera;
mod hittable;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;
mod material;

use rtweekend::random_float;
use camera::Camera;
use hittable::{Hittable, HittableList};
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Vec3, Point3};
use material::Material;




use image::{ImageBuffer, Rgb};

fn main() -> std::io::Result<()> {
    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let samples_per_pixel: u64 = 50;
    let image_width: u16 = 400;
    let image_height: u16 = (image_width as f64 / aspect_ratio) as u16;
    let max_depth = 50;

    //World
    let mut world = HittableList {
        objects: Vec::new(),
    };
    let material_ground = Material::Lambertian(Color::new(0.5, 0.5, 0.5));
    
    let ground_sphere = 
    Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat: material_ground,
    };

    world.objects.push(hittable::Hittable::S(ground_sphere));

    for i in  -11..11{
        for j in -11..11{
            let choose_mat = random_float();
            let center = Point3::new(i as f64 + 0.9 * random_float(), 0.2, j as f64 + 0.9 * random_float());
            if choose_mat< 0.8 {
                let albedo = Color::random_vec3s(0.0,1.0) * Color::random_vec3s(0.0,1.0);
                let sphere_material = Material::Lambertian(albedo);
                world.objects.push(Hittable::S(Sphere {
                    center,
                    radius: 0.2,
                    mat: sphere_material,
                }));
            }else if choose_mat < 0.95 {
                let albedo = Color::random_vec3s(0.5,1.0);
                let fuzz = random_float() * 0.5;
                let sphere_material = Material::Metal{color: albedo, fuzz};
                world.objects.push(Hittable::S(Sphere {
                    center,
                    radius: 0.2,
                    mat: sphere_material,
                }));
            }else{
                let sphere_material = Material::Dielectric{ir: 1.5};
                world.objects.push(Hittable::S(Sphere {
                    center,
                    radius: 0.2,
                    mat: sphere_material,
                }));
            }
        }

    }
    
    let material1 = Material::Dielectric{ir: 1.5};
    world.objects.push(Hittable::S(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: material1,
    }));
    let material2 = Material::Lambertian(Color::new(0.4, 0.2, 0.1));
    world.objects.push(Hittable::S(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: material2,
    }));
    let material3 = Material::Metal {
        color: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    world.objects.push(Hittable::S(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: material3,
    }));

    // Camera

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let defocus_angle = 0.1;


    let camera: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0, 
        aspect_ratio,
        defocus_angle,
        dist_to_focus);

    //Render
    let mut image = ImageBuffer::new(image_width as u32, image_height as u32);
    for i in (0..image_height).rev() {
        for j in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u: f64 = (j as f64 + random_float()) / ((image_width - 1) as f64);
                let v: f64 = ((image_height - 1 - i) as f64 + random_float()) / ((image_height - 1) as f64);
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(r, &world, max_depth);
            }
            write_colors(&mut image, pixel_color, samples_per_pixel,j as u32,i as u32);
        }
    }
    let _ = image.save("out.png");
    Ok(())
}

fn write_colors(
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    pixel_color: Color,
    samples_per_pixel: u64,
    x: u32,
    y: u32,
)  {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    //divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let ir = (256.0 * r.min(0.999)) as u8;
    let ig = (256.0 * g.min(0.999)) as u8;
    let ib = (256.0 * b.min(0.999)) as u8;


    image.put_pixel(x, y, Rgb([ir, ig, ib]));
    
}




fn ray_color(r: Ray, world: &hittable::HittableList, depth: u32) -> Vec3 {
    let c = Color::new(0.0, 0.0, 0.0);

    if depth <= 0 {
        return Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let mut scattered = Ray::new(c, c);
        let mut attenuation = c;
        if rec.mat.scatter(r, rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Color::new(0.5, 0.7, 1.0) * t + Color::new(1.0, 1.0, 1.0) * (1.0 - t);
}

