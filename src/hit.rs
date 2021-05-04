use crate::materials::Material;
use crate::ray::Ray;
use crate::vec::Vec3;
use std::rc::Rc;

pub struct Hit {
    pub t: f32,  // 光击中物体时 t 的大小
    pub p: Vec3, // 击中的点的位置
    pub normal: Option<Vec3>,
    pub front_face: Option<bool>,
    pub material: Option<Rc<dyn Material>>,
}

impl Hit {
    pub fn new(t: f32, p: Vec3) -> Hit {
        Hit {
            t,
            p,
            normal: None,
            front_face: None,
            material: None,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        self.front_face = Some(front_face);
        self.normal = Some(normal);
    }
}

pub trait Hittable {
    fn hit(&self, t_min: f32, t_max: f32, r: &Ray) -> Option<Hit>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, t_min: f32, t_max: f32, ray: &Ray) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let t = root;
        let p = ray.at(root);
        let mut hit = Hit::new(t, p);
        let outward_normal = (hit.p - self.center) / self.radius;
        hit.set_face_normal(ray, outward_normal);
        hit.material = Some(self.material.clone());
        Some(hit)
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, t_min: f32, t_max: f32, r: &Ray) -> Option<Hit> {
        let mut temp_rec: Option<Hit> = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            let result = object.hit(t_min, closest_so_far, r);
            if let Some(rec) = result {
                // 找到距离最近的物体
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }
        temp_rec
    }
}
