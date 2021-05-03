mod ray;
mod vec;

use crate::vec::Vec3;
use indicatif::ProgressBar;
use lodepng::RGB;
use std::path::Path;

// struct Pixel(usize, usize, usize);
fn main() {
    // Image
    const WIDTH: usize = 256;
    const HEIGHT: usize = 256;
    let mut pixels: Vec<RGB<u8>> = Vec::with_capacity(WIDTH * HEIGHT);
    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let r: f32 = (i as f32) / (WIDTH - 1) as f32;
            let g: f32 = (j as f32) / (HEIGHT - 1) as f32;
            let b: f32 = 0.25;
            let pixel_color = Vec3(r, g, b);
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
