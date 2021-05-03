mod camera;
mod hit;
mod ray;
mod utils;
mod vec;

use crate::camera::Camera;
use crate::hit::{Hittable, HittableList, Sphere};
use crate::ray::Ray;
use crate::vec::Vec3;
use indicatif::ProgressBar;
use lodepng::RGB;
use rand::Rng;
use std::path::Path;

pub fn ray_color(ray: &Ray, world: &HittableList, depth: usize) -> Vec3 {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Vec3(0.0, 0.0, 0.0);
    }

    let result = world.hit(0.0, f32::MAX, ray);
    if let Some(rec) = result {
        let target: Vec3 = rec.p + rec.normal.unwrap() + Vec3::rand_in_unit_sphere();
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
    }
    const WHITE: Vec3 = Vec3(1.0, 1.0, 1.0);
    const SKY_BLUE: Vec3 = Vec3(0.5, 0.7, 1.0);
    let unit_direction = ray.direction.to_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * WHITE + t * SKY_BLUE
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const WIDTH: usize = 400;
    const HEIGHT: usize = (WIDTH as f32 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;

    // World
    let mut world = HittableList::new();
    let sphere1 = Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    let sphere2 = Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
    };
    world.add(Box::new(sphere1));
    world.add(Box::new(sphere2));

    // Camera
    let cam = Camera::new();

    // random f32
    let mut rng = rand::thread_rng();

    // Progress bar
    let bar = ProgressBar::new(HEIGHT as u64);

    // Render
    let mut pixels: Vec<RGB<u8>> = Vec::with_capacity(WIDTH * HEIGHT);
    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let mut pixel_color = Vec3(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u: f32 = (i as f32 + rng.gen_range(0.0..1.0)) / (WIDTH - 1) as f32;
                let v: f32 = (j as f32 + rng.gen_range(0.0..1.0)) / (HEIGHT - 1) as f32;
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, MAX_DEPTH);
            }
            let pixel = pixel_color.to_rgb_sampled(SAMPLES_PER_PIXEL);
            pixels.push(pixel);
        }
        bar.inc(1);
    }

    bar.finish();

    let path = &Path::new("image.png");

    if let Err(e) = lodepng::encode_file(path, &pixels, WIDTH, HEIGHT, lodepng::ColorType::RGB, 8) {
        panic!("failed to write png: {:?}", e);
    }

    println!("Written to {}", path.display());
}
