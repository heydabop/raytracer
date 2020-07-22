use super::aabb::AABB;
use super::hit::{Hit, Hittable};
use super::material::{Lambertian, MaterialWritable};
use super::ray::Ray;
use super::vec3::Vec3;
use std::rc::Rc;

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<dyn MaterialWritable>,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            center: Vec3::new(),
            radius: 0.0,
            material: Rc::new(Lambertian::new(Vec3::from_xyz(0.5, 0.5, 0.5))),
        }
    }

    fn compute_hit(&self, r: &Ray, t: f64) -> Option<Hit> {
        let point = r.at(t);
        let mut normal = (&point - &self.center) / self.radius;
        let front_face = if r.direction.dot(&normal) > 0.0 {
            // ray is coming from inside the sphere
            normal = -normal;
            false
        } else {
            true
        };
        Some(Hit {
            point,
            normal: normal.unit_vector(),
            t,
            front_face,
            material: Rc::clone(&self.material),
        })
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new()
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = &r.origin - &self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = (-self.radius).mul_add(self.radius, oc.length_squared());
        let discriminant = (-a).mul_add(c, half_b * half_b);

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

        None
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(AABB {
            min: &self.center - Vec3::from_xyz(self.radius, self.radius, self.radius),
            max: &self.center + Vec3::from_xyz(self.radius, self.radius, self.radius),
        })
    }
}

impl Hittable for &Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        (*self).hit(r, t_min, t_max)
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        (*self).bounding_box(t0, t1)
    }
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod test {
    use super::{Hit, Hittable, Lambertian, Ray, Rc, Sphere, Vec3};

    #[test]
    fn hit() {
        let s = &Sphere {
            center: Vec3::from_xyz(0.0, 0.0, 1.0),
            radius: 0.5,
            material: Rc::new(Lambertian::new(Vec3::from_xyz(0.5, 0.5, 0.5))),
        };
        let hit_ray = Ray {
            origin: Vec3::new(),
            direction: Vec3::from_xyz(0.2, 0.3, 1.0),
            time: 0.0,
        };
        let inside_hit_ray = Ray {
            origin: Vec3::from_xyz(0.0, 0.0, 1.0),
            direction: Vec3::from_xyz(0.0, 1.0, 0.0),
            time: 0.0,
        };
        let miss_ray = Ray {
            origin: Vec3::new(),
            direction: Vec3::from_xyz(0.0, 0.7, 1.0),
            time: 0.0,
        };
        assert_eq!(
            s.hit(&hit_ray, 0.0, 2.0),
            Some(Hit {
                point: Vec3 {
                    x: 0.10787389667339245,
                    y: 0.16181084501008866,
                    z: 0.5393694833669622
                },
                normal: Vec3 {
                    x: 0.2157477933467849,
                    y: 0.3236216900201773,
                    z: -0.9212610332660756
                },
                t: 0.5393694833669622,
                front_face: true,
                material: Rc::new(Lambertian::new(Vec3::from_xyz(0.5, 0.5, 0.5))),
            })
        );
        assert_eq!(
            s.hit(&inside_hit_ray, 0.0, 1.0),
            Some(Hit {
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
                material: Rc::new(Lambertian::new(Vec3::from_xyz(0.5, 0.5, 0.5))),
            })
        );
        assert_eq!(s.hit(&hit_ray, 0.0, 0.5), None);
        assert_eq!(s.hit(&miss_ray, 0.0, 10.0), None);
    }
}
