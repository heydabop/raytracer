use super::vec3::Vec3;

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
        if self.hit_sphere(&Vec3::init(0.0, 0.0, -1.0), 0.5) {
            return Vec3::init(1.0, 0.0, 0.0);
        }

        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);

        Vec3::init(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::init(0.5, 0.7, 1.0) * t
    }

    fn hit_sphere(&self, center: &Vec3, radius: f64) -> bool {
        let oc = &self.origin - center;
        let a = self.direction.dot(&self.direction);
        let b = oc.dot(&self.direction) * 2.0;
        let c = oc.dot(&oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;

        discriminant > 0.0
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
