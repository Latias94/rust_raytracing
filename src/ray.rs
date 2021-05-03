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

    pub fn ray_color(&self) -> Vec3 {
        const WHITE: Vec3 = Vec3(1.0, 1.0, 1.0);
        const SKY_BLUE: Vec3 = Vec3(0.5, 0.7, 1.0);
        let sphere_center = Vec3(0.0, 0.0, -1.0);
        let t = self.hit_sphere(sphere_center, 0.5);
        if t > 0.0 {
            let normal = (self.at(t) - sphere_center).to_unit_vector();
            return 0.5 * Vec3(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
        }

        let unit_direction = self.direction.to_unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * WHITE + t * SKY_BLUE
    }

    pub fn hit_sphere(&self, center: Vec3, radius: f32) -> f32 {
        let oc = self.origin - center;
        let a = self.direction.dot(self.direction);
        let b = 2.0 * oc.dot(self.direction);
        let c = oc.dot(oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            -1.0
        } else {
            (-b - discriminant.sqrt()) / (2.0 * a)
        }
    }
}
