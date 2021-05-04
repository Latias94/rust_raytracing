use crate::hit::Hit;
use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Scatter {
    pub attenuation: Vec3,
    pub ray: Ray,
}

// 材料将告诉我们光线如何与表面相互作用
pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &Hit) -> Option<Scatter>;
}

#[derive(Debug)]
pub struct Lambertian {
    // 反射率
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(color: Vec3) -> Lambertian {
        Lambertian { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &Hit) -> Option<Scatter> {
        let normal = rec.normal.unwrap();
        let mut scatter_direction = normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = normal;
        }
        Some(Scatter {
            attenuation: self.albedo,
            ray: Ray {
                origin: rec.p,
                direction: scatter_direction,
            },
        })
    }
}

#[derive(Debug)]
pub struct Metal {
    // 反射率
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(color: Vec3, f: f32) -> Metal {
        Metal {
            albedo: color,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &Hit) -> Option<Scatter> {
        let reflected = Vec3::reflect(&r_in.direction.to_unit_vector(), &rec.normal.unwrap());
        let scattered = Ray {
            origin: rec.p,
            direction: reflected + self.fuzz * Vec3::rand_in_unit_sphere(),
        };
        let same_direction = scattered.direction.dot(rec.normal.unwrap()) > 0.0;
        if same_direction {
            Some(Scatter {
                attenuation: self.albedo,
                ray: scattered,
            })
        } else {
            None
        }
    }
}
