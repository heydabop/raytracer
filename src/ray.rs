use super::hit::*;
use super::sphere::Sphere;
use super::vec3::Vec3;
use std::default::Default;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    #[allow(dead_code)]
    pub const fn new() -> Self {
        Self {
            origin: Vec3::new(),
            direction: Vec3::new(),
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        &self.origin + &self.direction * t
    }

    pub fn color(&self) -> Vec3 {
        let s = &Sphere {
            center: Vec3::init(0.0, 0.0, -1.0),
            radius: 0.5,
        };
        let hit = &s.hit(self, 0.0, 10.0);
        if let Hit::Hit {
            point: _,
            normal,
            t: _,
        } = hit
        {
            return Vec3::init(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5;
        }

        // miss; return background gradient

        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);

        Vec3::init(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::init(0.5, 0.7, 1.0) * t
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray::new()
    }
}

#[cfg(test)]
mod test {
    use super::{Ray, Vec3};

    #[test]
    fn at() {
        let a = Ray {
            origin: Vec3::init(0.0, -1.0, -2.0),
            direction: Vec3::init(1.0, 2.0, 3.0),
        };
        let b = &a.at(2.5);
        assert_eq!(b, &Vec3::init(2.5, 4.0, 5.5));
    }
}
