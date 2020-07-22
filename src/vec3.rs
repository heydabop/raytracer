use rand::Rng;
use std::fmt;
use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    #[allow(dead_code)]
    pub const fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[allow(dead_code)]
    pub const fn from_xyz(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn random<T: Rng>(rng: &mut T, min: f64, max: f64) -> Self {
        Self::from_xyz(
            rng.gen_range(min, max),
            rng.gen_range(min, max),
            rng.gen_range(min, max),
        )
    }

    pub fn random_in_unit_sphere<T: Rng>(mut rng: &mut T) -> Self {
        loop {
            let v = Self::random(&mut rng, -1.0, 1.0);
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_unit_disk<T: Rng>(rng: &mut T) -> Self {
        loop {
            let v = Self::from_xyz(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.), 0.0);
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_unit_vector<T: Rng>(rng: &mut T) -> Self {
        let a: f64 = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
        let z: f64 = rng.gen_range(-1.0, 1.0);
        let r = (z.mul_add(-z, 1.0)).sqrt();

        Self::from_xyz(r * a.cos(), r * a.sin(), z)
    }

    pub fn random_in_hemisphere<T: Rng>(mut rng: &mut T, normal: &Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere(&mut rng);
        if in_unit_sphere.dot(normal) > 0.0 {
            // same direction (so same hemisphere) as normal
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub const fn r(&self) -> f64 {
        self.x
    }
    pub const fn g(&self) -> f64 {
        self.y
    }
    pub const fn b(&self) -> f64 {
        self.z
    }

    pub fn is_valid_color(&self, samples: u32) -> bool {
        let scale = 1.0 / f64::from(samples);
        let x = self.x * scale;
        let y = self.y * scale;
        let z = self.z * scale;

        0.0 <= x && x <= 1.0 && 0.0 <= y && y <= 1.0 && 0.0 <= z && z <= 1.0
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        // x*x + y*y + z*z
        self.x.mul_add(rhs.x, self.y.mul_add(rhs.y, self.z * rhs.z))
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y.mul_add(rhs.z, -self.z * rhs.y),
            y: self.z.mul_add(rhs.x, -self.x * rhs.z),
            z: self.x.mul_add(rhs.y, -self.y * rhs.x),
        }
    }

    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        self - &(normal * 2.0 * self.dot(&normal))
    }

    pub fn refract(&self, n: &Self, eta_ratio: f64) -> Self {
        let cos_theta = -self.dot(n);
        let r_out_parallel = (self + n * cos_theta) * eta_ratio;
        let r_out_perpindicular = n * -(1.0 - r_out_parallel.length_squared()).sqrt();

        r_out_parallel + r_out_perpindicular
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new()
    }
}

impl fmt::Display for &Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.x, self.y, self.z)
    }
}

impl ops::Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        self + &rhs
    }
}

impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl ops::Index<u8> for &Vec3 {
    type Output = f64;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index {} out of bounds", index),
        }
    }
}

impl ops::Index<u8> for Vec3 {
    type Output = f64;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index {} out of bounds", index),
        }
    }
}

impl ops::IndexMut<u8> for Vec3 {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index {} out of bounds", index),
        }
    }
}

impl ops::Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl std::cmp::PartialEq for Vec3 {
    fn eq(&self, rhs: &Self) -> bool {
        (self.x - rhs.x).abs() < f64::EPSILON
            && (self.y - rhs.y).abs() < f64::EPSILON
            && (self.z - rhs.z).abs() < f64::EPSILON
    }
}

impl ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self - &rhs
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        &self - rhs
    }
}

impl ops::SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod test {
    use super::Vec3;

    #[test]
    fn add() {
        let a = Vec3::from_xyz(1.0, 2.0, 3.0);
        let b = Vec3::from_xyz(4.0, 5.0, 6.0);
        let c = b.clone();
        let d = &a + &b;
        let e = &a + b;
        let f = a + c;
        let res = Vec3::from_xyz(5.0, 7.0, 9.0);
        assert_eq!(d, res);
        assert_eq!(e, res);
        assert_eq!(f, res);
    }

    #[test]
    fn add_assign() {
        let mut a = Vec3::from_xyz(1.0, 2.0, 3.0);
        let b = Vec3::from_xyz(4.0, 5.0, 6.0);
        a += &b;
        assert_eq!(a, Vec3::from_xyz(5.0, 7.0, 9.0));
    }

    #[test]
    fn cross() {
        let a = Vec3::from_xyz(1.0, 2.0, 3.0);
        let b = Vec3::from_xyz(4.0, 5.0, 7.0);
        let c = a.cross(&b);
        assert_eq!(c, Vec3::from_xyz(-1.0, 5.0, -3.0));
    }

    #[test]
    fn div() {
        let a = Vec3::from_xyz(1.0, 2.0, 3.0);
        let b = &a / 2.0;
        let c = a / 2.0;
        let res = Vec3::from_xyz(0.5, 1.0, 1.5);
        assert_eq!(b, res);
        assert_eq!(c, res);
    }

    #[test]
    fn div_assign() {
        let mut a = Vec3::from_xyz(1.0, 2.0, 3.0);
        a /= 2.0;
        assert_eq!(a, Vec3::from_xyz(0.5, 1.0, 1.5));
    }

    #[test]
    fn dot() {
        let a = Vec3::from_xyz(1.0, 2.0, 3.0);
        let b = Vec3::from_xyz(4.0, 5.0, 6.0);
        assert!((a.dot(&b) - 32.0).abs() < f64::EPSILON);
    }

    #[test]
    fn fmt() {
        let s = format!("{}", &Vec3::from_xyz(1.0, 2.0, 3.0));
        assert_eq!(s, "(1 2 3)");
    }

    #[test]
    fn mul() {
        let a = Vec3::from_xyz(1.0, 2.0, 3.0);
        let b = Vec3::from_xyz(4.0, 5.0, 6.0);
        let c = &a * &b;
        let d = a * b;
        let res = Vec3::from_xyz(4.0, 10.0, 18.0);
        assert_eq!(c, res);
        assert_eq!(d, res);
    }

    #[test]
    fn mul_f64() {
        let a = Vec3::from_xyz(1.0, 2.0, 3.0);
        let b = &a * 2.0;
        let c = a * 2.0;
        let res = Vec3::from_xyz(2.0, 4.0, 6.0);
        assert_eq!(b, res);
        assert_eq!(c, res);
    }

    #[test]
    fn mul_assign() {
        let mut a = Vec3::from_xyz(1.0, 2.0, 3.0);
        a *= 2.0;
        assert_eq!(a, Vec3::from_xyz(2.0, 4.0, 6.0));
    }

    #[test]
    fn neg() {
        let a = Vec3::from_xyz(1.0, 2.0, 3.0);
        let res = Vec3::from_xyz(-1.0, -2.0, -3.0);
        assert_eq!(-&a, res);
        assert_eq!(-a, res);
    }

    #[test]
    fn sub() {
        let a = Vec3::from_xyz(1.0, 2.0, 3.0);
        let b = Vec3::from_xyz(1.0, 3.0, 5.0);
        let c = &a - &b;
        let d = a - b;
        let res = Vec3::from_xyz(0.0, -1.0, -2.0);
        assert_eq!(c, res);
        assert_eq!(d, res);
    }

    #[test]
    fn sub_assign() {
        let mut a = Vec3::from_xyz(1.0, 2.0, 3.0);
        let b = Vec3::from_xyz(1.0, 3.0, 5.0);
        a -= &b;
        assert_eq!(a, Vec3::from_xyz(0.0, -1.0, -2.0));
    }

    #[test]
    fn rgb() {
        let a = Vec3::from_xyz(1.0, 2.0, 3.0);
        assert!((a.r() - 1.0).abs() < f64::EPSILON);
        assert!((a.g() - 2.0).abs() < f64::EPSILON);
        assert!((a.b() - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn index() {
        let mut a = Vec3::from_xyz(4.0, 5.0, 6.0);
        assert!((a[0] - 4.0).abs() < f64::EPSILON);
        assert!((a[1] - 5.0).abs() < f64::EPSILON);
        assert!((a[2] - 6.0).abs() < f64::EPSILON);
        a[0] = 7.0;
        a[1] = 8.0;
        a[2] = 9.0;
        assert!((a[0] - 7.0).abs() < f64::EPSILON);
        assert!((a[1] - 8.0).abs() < f64::EPSILON);
        assert!((a[2] - 9.0).abs() < f64::EPSILON);
    }

    #[test]
    fn length() {
        let a = Vec3::from_xyz(2.0, 3.0, 4.0);
        assert!((a.length_squared() - 29.0).abs() < f64::EPSILON);
        assert!((a.length() - 29.0_f64.sqrt()).abs() < f64::EPSILON);
    }

    #[test]
    fn is_valid_color() {
        let valid = Vec3::from_xyz(9.0, 10.0, 0.1);
        let invalid = Vec3::from_xyz(9.0, 10.2, 0.0);
        assert!(valid.is_valid_color(10));
        assert!(!invalid.is_valid_color(10));
    }

    #[test]
    fn unit_vector() {
        let a = Vec3::from_xyz(5.0, 4.0, 3.0);
        assert_eq!(
            a.unit_vector(),
            Vec3::from_xyz(0.7071067811865475, 0.565685424949238, 0.4242640687119285)
        );
    }

    #[test]
    fn reflect() {
        let a = Vec3::from_xyz(1.0, 2.0, 3.0);
        let normal = Vec3::from_xyz(0.218218, 0.436436, 0.872872);
        assert_eq!(
            a.reflect(&normal),
            Vec3::from_xyz(
                -0.6190492478159999,
                -1.2380984956319998,
                -3.4761969912639996
            )
        );
    }

    #[test]
    fn refract() {
        let a = Vec3::from_xyz(1.0, 2.0, 3.0).unit_vector();
        assert_eq!(
            a.refract(&Vec3::from_xyz(0.1, 1.0, 0.1).unit_vector(), 1.0 / 1.3),
            Vec3::from_xyz(0.07757118074341529, -0.8689727581807664, 0.4887423221471451)
        );
    }
}
