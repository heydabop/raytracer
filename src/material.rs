use super::hit::HitData;
use super::ray::Ray;
use super::vec3::Vec3;
use std::fmt;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Vec3,
}

pub trait Material {
    // Returns (if ray scatters) new scattered ray and attenuation of ray
    fn scatter(&self, r_in: &Ray, hit: &HitData) -> Option<Scatter>;
}

#[allow(clippy::module_name_repetitions)]
pub trait MaterialWritable: Material + fmt::Debug {}

// Lambertian

#[derive(Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    #[allow(dead_code)]
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
    fn scatter(&self, _: &Ray, hit: &HitData) -> Option<Scatter> {
        let scatter_direction = &hit.normal + Vec3::random_unit_vector();
        let scattered_ray = Ray {
            origin: hit.point.clone(),
            direction: scatter_direction,
        };

        Some(Scatter {
            ray: scattered_ray,
            attenuation: self.albedo.clone(),
        })
    }
}

impl MaterialWritable for Lambertian {}

// Metal

#[derive(Debug)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    #[allow(dead_code)]
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Default for Metal {
    fn default() -> Self {
        Self {
            albedo: Vec3::default(),
            fuzz: 0.0,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &HitData) -> Option<Scatter> {
        let reflected = r_in.direction.unit_vector().reflect(&hit.normal);
        if reflected.dot(&hit.normal) > 0.0 {
            let scattered = Ray {
                origin: hit.point.clone(),
                direction: reflected + Vec3::random_in_unit_sphere() * self.fuzz,
            };
            return Some(Scatter {
                ray: scattered,
                attenuation: self.albedo.clone(),
            });
        }

        None
    }
}

impl MaterialWritable for Metal {}
