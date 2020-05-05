use super::hit::*;
use super::ray::Ray;
use super::vec3::Vec3;
use std::default::Default;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub const fn new() -> Self {
        Sphere {
            center: Vec3::new(),
            radius: 0.0,
        }
    }

    fn compute_hit(&self, r: &Ray, t: f64) -> Hit {
        let point = r.at(t);
        Hit::Hit {
            point: point.clone(),
            normal: (&point - &self.center) / self.radius,
            t,
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new()
    }
}

impl Hittable for &Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Hit {
        let oc = &r.origin - &self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut t = (-half_b - root) / a;
            if t < t_max && t > t_min {
                return self.compute_hit(r, t);
            }
            t = (-half_b + root) / a;
            if t < t_max && t > t_min {
                return self.compute_hit(r, t);
            }
        }

        Hit::Miss
    }
}
