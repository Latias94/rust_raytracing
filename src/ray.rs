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

    /// 根据 y 值将蓝白做了个线性插值的混合, 我们这里把射线做了个单位化, 以保证 y 的取值范围
    // (-1.0 < y < 1.0) 。因为我们使用 y 轴做渐变, 所以你可以看到这个蓝白渐变也是竖直的。
    pub fn ray_color(&self) -> Vec3 {
        let unit_direction = self.direction.to_unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
    }
}
