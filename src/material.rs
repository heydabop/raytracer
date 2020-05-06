use super::hit::HitData;
use super::ray::Ray;
use super::vec3::Vec3;
use std::fmt;

pub trait Material {
    // Returns new scattered ray and attenuation of ray
    fn scatter(&self, r_in: &Ray, hit: &HitData) -> (Ray, Vec3);
}

#[allow(clippy::module_name_repetitions)]
pub trait MaterialWritable: Material + fmt::Debug {}

#[derive(Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Self {
            albedo: Vec3::default(),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitData) -> (Ray, Vec3) {
        let scatter_direction = &hit.normal + Vec3::random_unit_vector();
        let scattered_ray = Ray {
            origin: hit.point.clone(),
            direction: scatter_direction,
        };

        (scattered_ray, self.albedo.clone())
    }
}

impl MaterialWritable for Lambertian {}
