use super::vec3::Vec3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

pub trait TextureWritable: Texture + std::fmt::Debug {}

#[derive(Debug)]
pub struct SolidColor {
    pub color: Vec3,
}

impl SolidColor {
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            color: Vec3 { x: r, y: g, z: b },
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.color
    }
}

impl TextureWritable for SolidColor {}

#[derive(Debug)]
pub struct CheckerTexture {
    pub odd: Box<dyn TextureWritable>,
    pub even: Box<dyn TextureWritable>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

impl TextureWritable for CheckerTexture {}
