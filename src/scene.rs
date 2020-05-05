use super::hit::{Hit, Hittable};
use super::ray::Ray;

pub struct Scene {
    objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, o: Box<dyn Hittable>) {
        self.objects.push(o);
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

impl Hittable for Scene {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Hit {
        let mut hit = Hit::Miss;
        let mut closest = t_max;

        for obj in &self.objects {
            if let Hit::Hit(obj_hit) = obj.hit(r, t_min, closest) {
                closest = obj_hit.t;
                hit = Hit::Hit(obj_hit);
            }
        }

        hit
    }
}

impl Hittable for &Scene {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Hit {
        (*self).hit(r, t_min, t_max)
    }
}
