use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn surrounding_box(&self, b1: &Self) -> Self {
        Self {
            min: Vec3 {
                x: self.min.x.min(b1.min.x),
                y: self.min.y.min(b1.min.y),
                z: self.min.z.min(b1.min.z),
            },
            max: Vec3 {
                x: self.max.x.max(b1.max.x),
                y: self.max.y.max(b1.max.y),
                z: self.max.z.max(b1.max.z),
            },
        }
    }

    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let mut t0 = (self.min[a] - r.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - r.origin[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}
