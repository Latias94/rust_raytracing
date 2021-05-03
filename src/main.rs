mod ray;
mod vec;

use crate::ray::Ray;
use crate::vec::Vec3;
use lodepng::RGB;
use std::path::Path;

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const WIDTH: usize = 400;
    const HEIGHT: usize = (WIDTH as f32 / ASPECT_RATIO) as usize;

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
            let pixel_color = r.ray_color();
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
