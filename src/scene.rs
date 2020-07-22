use super::aabb::AABB;
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut hit = None;
        let mut closest = t_max;

        for obj in &self.objects {
            if let Some(obj_hit) = obj.hit(r, t_min, closest) {
                closest = obj_hit.t;
                hit = Some(obj_hit);
            }
        }

        hit
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            // nothing to box
            return None;
        }

        // build up a bounding box of entire scene
        let mut bounding_box: Option<AABB> = None;

        for o in &self.objects {
            // compute this object's bounding box
            if let Some(o_box) = o.bounding_box(t0, t1) {
                // if this object has a BB, add it to the scene's box (if we have one), otherwise this first box is now our scene's box
                bounding_box = if let Some(t_box) = bounding_box {
                    Some(t_box.surrounding_box(&o_box))
                } else {
                    Some(o_box)
                };
            } else {
                // if any object doesn't have a bounding box then neither does this scene
                return None;
            }
        }

        bounding_box
    }
}

impl Hittable for &Scene {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        (*self).hit(r, t_min, t_max)
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        (*self).bounding_box(t0, t1)
    }
}
