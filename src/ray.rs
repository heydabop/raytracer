use super::vec3::Vec3;

#[derive(Debug, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
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
        &self.origin + &(&self.direction * t)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn at() {
        let a = Ray {
            origin: Vec3(0.0, -1.0, -2.0),
            direction: Vec3(1.0, 2.0, 3.0),
        };
        let b = &a.at(2.5);
        assert_eq!(b, &Vec3(2.5, 4.0, 5.5));
    }
}
