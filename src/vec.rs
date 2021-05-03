use lodepng::RGB;
use rand::distributions::{Distribution, Standard};
use rand::{random, Rng};
use std::ops::*;

#[derive(Copy, Clone, Debug)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn x(&self) -> f32 {
        self.0
    }
    pub fn y(&self) -> f32 {
        self.1
    }
    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn squared_length(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn to_u8(&self) -> [u8; 3] {
        fn u(f: f32) -> u8 {
            if f < 0.0 {
                0
            } else if f >= 1.0 {
                255
            } else {
                (f * 255.999) as i32 as u8
            }
        }
        [u(self.0), u(self.1), u(self.2)]
    }

    pub fn to_rgb(&self) -> RGB<u8> {
        let rgb = &self.to_u8();
        RGB::new(rgb[0], rgb[1], rgb[2])
    }

    pub fn to_rgb_sampled(&self, samples_per_pixel: usize) -> RGB<u8> {
        let scale = 1.0 / (samples_per_pixel as f32);
        // Divide the color by the number of samples and gamma-correct for gamma=2.0.
        let r = (scale * self.0 as f32).sqrt();
        let r = (256.0 * f32::clamp(r, 0.0, 0.999)) as u8;
        let g = (scale * self.1 as f32).sqrt();
        let g = (256.0 * f32::clamp(g, 0.0, 0.999)) as u8;
        let b = (scale * self.2 as f32).sqrt();
        let b = (256.0 * f32::clamp(b, 0.0, 0.999)) as u8;
        RGB::new(r, g, b)
    }

    pub fn to_unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self * other.0, self * other.1, self * other.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        (1.0 / other) * self
    }
}

impl Distribution<Vec3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3(rng.gen(), rng.gen(), rng.gen())
    }
}

impl Vec3 {
    // fn rand_in_range<R: Rng>(rng: &mut R, min: f32, max: f32) -> Vec3 {
    //     Vec3(
    //         rng.gen_range(min..max),
    //         rng.gen_range(min..max),
    //         rng.gen_range(min..max),
    //     )
    // }

    pub fn rand_in_unit_sphere() -> Vec3 {
        loop {
            let p = 2.0 * random::<Vec3>() - Vec3(1.0, 1.0, 1.0);
            if p.squared_length() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::rand_in_unit_sphere().to_unit_vector()
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::rand_in_unit_sphere();
        if in_unit_sphere.dot(*normal) > 0.0 {  // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
}
