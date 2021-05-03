use crate::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn hit_sphere(&self, center: Vec3, radius: f32) -> f32 {
        let oc = self.origin - center;
        let a = self.direction.dot(self.direction);
        let half_b = oc.dot(self.direction);
        let c = oc.dot(oc) - radius * radius;
        let discriminant = (half_b * half_b) - a * c;
        if discriminant < 0.0 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
    }
}
