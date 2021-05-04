mod camera;
mod hit;
mod materials;
mod ray;
mod utils;
mod vec;

use crate::camera::Camera;
use crate::hit::{Hittable, HittableList, Sphere};
use crate::materials::{Dielectric, Lambertian, Material, Metal};
use crate::ray::Ray;
use crate::vec::Vec3;
use indicatif::ProgressBar;
use lodepng::RGB;
use rand::Rng;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::path::Path;
use std::sync::Arc;

pub fn ray_color(ray: &Ray, world: &HittableList, depth: usize) -> Vec3 {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Vec3(0.0, 0.0, 0.0);
    }

    if let Some(mut rec) = world.hit(0.001, f32::MAX, ray) {
        let material = rec.material.unwrap();
        rec.material = None;
        return if let Some(scattered) = material.scatter(&ray, &rec) {
            scattered.attenuation * ray_color(&scattered.ray, &world, depth - 1)
        } else {
            Vec3(0.0, 0.0, 0.0)
        };
    }
    const WHITE: Vec3 = Vec3(1.0, 1.0, 1.0);
    const SKY_BLUE: Vec3 = Vec3(0.5, 0.7, 1.0);
    let unit_direction = ray.direction.to_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * WHITE + t * SKY_BLUE
}

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Arc::new(Lambertian::new(Vec3(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere {
        center: Vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    }));

    let random_double_in_range =
        |min: f32, max: f32| -> f32 { rand::thread_rng().gen_range(min..max) };
    let random_double = || -> f32 { random_double_in_range(0.0, 1.0) };

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Vec3(
                a as f32 + 0.9 * random_double(),
                0.2,
                b as f32 + 0.9 * random_double(),
            );
            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random_color() * Vec3::random_color();
                    sphere_material = Arc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_color_in_range(0.5, 1.0);
                    let fuzz = random_double_in_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                }
                world.add(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material: sphere_material,
                }));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere {
        center: Vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1,
    }));

    let material2 = Arc::new(Lambertian::new(Vec3(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere {
        center: Vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2,
    }));

    let material3 = Arc::new(Metal::new(Vec3(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere {
        center: Vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3,
    }));

    world
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    const WIDTH: usize = 1200;
    const HEIGHT: usize = (WIDTH as f32 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 500;
    const MAX_DEPTH: usize = 50;

    // World
    let world = random_scene();

    // Camera
    let look_from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture: f32 = 0.1;
    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Progress bar
    let bar = ProgressBar::new(HEIGHT as u64);

    // Render
    let white = (256.0 * f32::clamp(1.0, 0.0, 0.999)) as u8;
    let mut pixels: Vec<RGB<u8>> = vec![RGB::new(white, white, white); WIDTH * HEIGHT];

    let bands: Vec<(usize, &mut [RGB<u8>])> = pixels.chunks_mut(WIDTH).enumerate().collect();
    bands.into_par_iter().for_each(|(row, band)| {
        let height = HEIGHT - row;
        let mut rng = rand::thread_rng();
        for column in 0..WIDTH {
            let mut pixel_color = Vec3(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u: f32 = (column as f32 + rng.gen_range(0.0..1.0)) / (WIDTH - 1) as f32;
                let v: f32 = (height as f32 + rng.gen_range(0.0..1.0)) / (HEIGHT - 1) as f32;
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, MAX_DEPTH);
            }
            let pixel = pixel_color.to_rgb_sampled(SAMPLES_PER_PIXEL);
            band[column] = pixel;
        }
        bar.inc(1);
    });

    bar.finish();

    let path = &Path::new("image.png");

    if let Err(e) = lodepng::encode_file(path, &pixels, WIDTH, HEIGHT, lodepng::ColorType::RGB, 8) {
        panic!("failed to write png: {:?}", e);
    }

    println!("Written to {}", path.display());
}
