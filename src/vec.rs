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

    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
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

    /// Return true if the vector is close to zero in all dimensions.
    pub fn near_zero(&self) -> bool {
        let s: f32 = 1e-8;
        (libm::fabsf(self.0) < s) && (libm::fabsf(self.1) < s) && (libm::fabsf(self.2) < s)
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

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2.0 * v.dot(*n) * *n
    }

    pub fn refract(v: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = f32::min((-(*v)).dot(*n), 1.0);
        let r_out_perp = etai_over_etat * (*v + cos_theta * (*n));
        let r_out_parallel = libm::fabsf(1.0 - r_out_perp.length_squared()).sqrt() * -1.0 * *n;
        r_out_perp + r_out_parallel
    }
    pub fn other_refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = (-uv).dot(n);
        let r_out_parallel = etai_over_etat * (uv + cos_theta * n);
        let r_out_perp = -(1.0 - r_out_parallel.length_squared()).sqrt() * n;
        r_out_parallel + r_out_perp
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let gen_range = || -> f32 { rand::thread_rng().gen_range(-1.0..1.0) };
        loop {
            let p = Vec3(gen_range(), gen_range(), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_color() -> Vec3 {
        Vec3::random_color_in_range(0.0, 1.0)
    }

    pub fn random_color_in_range(min: f32, max: f32) -> Vec3 {
        let gen_range = || -> f32 { rand::thread_rng().gen_range(min..max) };
        Vec3(gen_range(), gen_range(), gen_range())
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = 2.0 * random::<Vec3>() - Vec3(1.0, 1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().to_unit_vector()
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(*normal) > 0.0 {
            // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
}
