use lodepng::RGB;
use rand::Rng;
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

impl Vec3 {
    fn rand<R: Rng>(rng: &mut R) -> Vec3 {
        Vec3(rng.gen(), rng.gen(), rng.gen())
    }
}
