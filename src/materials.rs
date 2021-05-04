use crate::hit::Hit;
use crate::ray::Ray;
use crate::vec::Vec3;
use rand::random;

#[derive(Clone, Copy, Debug)]
pub struct Scatter {
    pub attenuation: Vec3,
    pub ray: Ray,
}

// 材料将告诉我们光线如何与表面相互作用
pub trait Material: Send + Sync {
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
            direction: reflected + self.fuzz * Vec3::random_in_unit_sphere(),
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

/// 绝缘体
/// 透明的材料，例如水，玻璃和钻石
/// 当光线击中这类材料时，一条光线会分成两条，一条发生反射，一条发生折射
/// 我们会采取这样的策略：每次光线与物体相交时，要么反射要么折射，一次只发生一种情况，随机选取。
/// 反正最后采样次数多，会给这些结果取个平均值。
#[derive(Debug)]
pub struct Dielectric {
    // Index of Refraction
    pub ir: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &Hit) -> Option<Scatter> {
        let attenuation = Vec3(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face.unwrap() {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.direction.to_unit_vector();

        let cos_theta = libm::fminf((-unit_direction).dot(rec.normal.unwrap()), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;
        if cannot_refract || (schlick(cos_theta, refraction_ratio) > random::<f32>()) {
            direction = Vec3::reflect(&unit_direction, &rec.normal.unwrap());
        } else {
            direction = Vec3::other_refract(unit_direction, rec.normal.unwrap(), refraction_ratio);
        }

        // let refracted = Vec3::refract(&unit_direction, &rec.normal.unwrap(), refraction_ratio);
        Some(Scatter {
            attenuation,
            ray: Ray {
                origin: rec.p,
                direction,
            },
        })
    }
}

/// Christophe Schlick's approximation for the reflectivity of glass,
/// as a function of the angle of incidence and index of refraction.
fn schlick(cosine: f32, index: f32) -> f32 {
    let r0 = (1.0 - index) / (1.0 + index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
}
