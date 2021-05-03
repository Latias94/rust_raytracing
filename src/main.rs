mod hit;
mod ray;
mod utils;
mod vec;

use crate::hit::{Hittable, HittableList, Sphere};
use crate::ray::Ray;
use crate::vec::Vec3;
use lodepng::RGB;
use std::path::Path;

pub fn ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
    let result = world.hit(0.0, f32::MAX, ray);
    if let Some(rec) = result {
        return 0.5 * (rec.normal.unwrap() + Vec3(1.0, 1.0, 1.0));
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
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = ASPECT_RATIO * viewport_height;
    // the distance between the projection plane and the projection point
    let focal_length: f32 = 1.0;

    let origin = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);

    // Render
    let mut pixels: Vec<RGB<u8>> = Vec::with_capacity(WIDTH * HEIGHT);
    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let u: f32 = (i as f32) / (WIDTH - 1) as f32;
            let v: f32 = (j as f32) / (HEIGHT - 1) as f32;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r, &world);
            let pixel = pixel_color.to_rgb();
            pixels.push(pixel);
        }
    }
    let path = &Path::new("image.png");

    if let Err(e) = lodepng::encode_file(path, &pixels, WIDTH, HEIGHT, lodepng::ColorType::RGB, 8) {
        panic!("failed to write png: {:?}", e);
    }

    println!("Written to {}", path.display());
}
