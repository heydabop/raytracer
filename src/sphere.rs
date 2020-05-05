use super::hit::{Hit, Hittable};
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
        let mut normal = (&point - &self.center) / self.radius;
        let front_face = if r.direction.dot(&normal) > 0.0 {
            // ray is coming from inside the sphere
            normal = -normal;
            false
        } else {
            true
        };
        Hit::Hit {
            point,
            normal,
            t,
            front_face,
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

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod test {
    use super::{Hit, Hittable, Ray, Sphere, Vec3};

    #[test]
    fn hit() {
        let s = &Sphere {
            center: Vec3::init(0.0, 0.0, 1.0),
            radius: 0.5,
        };
        let hit_ray = Ray {
            origin: Vec3::new(),
            direction: Vec3::init(0.2, 0.3, 1.0),
        };
        let inside_hit_ray = Ray {
            origin: Vec3::init(0.0, 0.0, 1.0),
            direction: Vec3::init(0.0, 1.0, 0.0),
        };
        let miss_ray = Ray {
            origin: Vec3::new(),
            direction: Vec3::init(0.0, 0.7, 1.0),
        };
        assert_eq!(
            s.hit(&hit_ray, 0.0, 2.0),
            Hit::Hit {
                point: Vec3 {
                    x: 0.10787389667339242,
                    y: 0.16181084501008863,
                    z: 0.5393694833669621
                },
                normal: Vec3 {
                    x: 0.21574779334678484,
                    y: 0.32362169002017727,
                    z: -0.9212610332660758
                },
                t: 0.5393694833669621,
                front_face: true,
            }
        );
        assert_eq!(
            s.hit(&inside_hit_ray, 0.0, 1.0),
            Hit::Hit {
                point: Vec3 {
                    x: 0.0,
                    y: 0.5,
                    z: 1.0,
                },
                normal: Vec3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                t: 0.5,
                front_face: false,
            }
        );
        assert_eq!(s.hit(&hit_ray, 0.0, 0.5), Hit::Miss);
        assert_eq!(s.hit(&miss_ray, 0.0, 10.0), Hit::Miss);
    }
}
