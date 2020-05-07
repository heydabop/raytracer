use super::hit::{Hit, Hittable};
use super::vec3::Vec3;
use rand_pcg::Pcg64Mcg;

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

    pub fn color<T: Hittable>(&self, hittable: &T, mut rng: &mut Pcg64Mcg, depth: u16) -> Vec3 {
        if depth == 0 {
            return Vec3::default();
        }

        if let Hit::Hit(hit) = hittable.hit(self, 0.001, f64::INFINITY) {
            if let Some(scatter) = hit.material.scatter(self, &mut rng, &hit) {
                return scatter.attenuation * scatter.ray.color(hittable, &mut rng, depth - 1);
            }
            return Vec3::new();
        }

        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::from_xyz(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::from_xyz(0.5, 0.7, 1.0) * t
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
            origin: Vec3::from_xyz(0.0, -1.0, -2.0),
            direction: Vec3::from_xyz(1.0, 2.0, 3.0),
        };
        let b = &a.at(2.5);
        assert_eq!(b, &Vec3::from_xyz(2.5, 4.0, 5.5));
    }
}
