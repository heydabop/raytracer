use std::fmt;
use std::ops;

#[derive(Clone, Debug, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    #[allow(dead_code)]
    pub const fn new() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }

    pub fn ppm_pixel(self) -> String {
        format!(
            "{} {} {}\n",
            (255.999 * self.0) as u8,
            (255.999 * self.1) as u8,
            (255.999 * self.2) as u8
        )
    }
}

impl ops::Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
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
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
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

impl fmt::Display for &Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.0, self.1, self.2)
    }
}

impl ops::Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
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
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
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
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl ops::SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(4.0, 5.0, 6.0);
        let c = b.clone();
        let d = &a + &b;
        let e = &a + b;
        let f = a + c;
        let res = Vec3(5.0, 7.0, 9.0);
        assert_eq!(d, res);
        assert_eq!(e, res);
        assert_eq!(f, res);
    }

    #[test]
    fn add_assign() {
        let mut a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(4.0, 5.0, 6.0);
        a += &b;
        assert_eq!(a, Vec3(5.0, 7.0, 9.0));
    }

    #[test]
    fn cross() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(4.0, 5.0, 7.0);
        let c = a.cross(&b);
        assert_eq!(c, Vec3(-1.0, 5.0, -3.0));
    }

    #[test]
    fn div() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = &a / 2.0;
        let c = a / 2.0;
        let res = Vec3(0.5, 1.0, 1.5);
        assert_eq!(b, res);
        assert_eq!(c, res);
    }

    #[test]
    fn div_assign() {
        let mut a = Vec3(1.0, 2.0, 3.0);
        a /= 2.0;
        assert_eq!(a, Vec3(0.5, 1.0, 1.5));
    }

    #[test]
    fn dot() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(4.0, 5.0, 6.0);
        assert_eq!(a.dot(&b), 32.0);
    }

    #[test]
    fn fmt() {
        let s = format!("{}", &Vec3(1.0, 2.0, 3.0));
        assert_eq!(s, "(1 2 3)");
    }

    #[test]
    fn mul() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(4.0, 5.0, 6.0);
        let c = &a * &b;
        let d = a * b;
        let res = Vec3(4.0, 10.0, 18.0);
        assert_eq!(c, res);
        assert_eq!(d, res);
    }

    #[test]
    fn mul_f64() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = &a * 2.0;
        let c = a * 2.0;
        let res = Vec3(2.0, 4.0, 6.0);
        assert_eq!(b, res);
        assert_eq!(c, res);
    }

    #[test]
    fn mul_assign() {
        let mut a = Vec3(1.0, 2.0, 3.0);
        a *= 2.0;
        assert_eq!(a, Vec3(2.0, 4.0, 6.0));
    }

    #[test]
    fn neg() {
        let a = Vec3(1.0, 2.0, 3.0);
        let res = Vec3(-1.0, -2.0, -3.0);
        assert_eq!(-&a, res);
        assert_eq!(-a, res);
    }

    #[test]
    fn sub() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(1.0, 3.0, 5.0);
        let c = &a - &b;
        let d = a - b;
        let res = Vec3(0.0, -1.0, -2.0);
        assert_eq!(c, res);
        assert_eq!(d, res);
    }

    #[test]
    fn sub_assign() {
        let mut a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(1.0, 3.0, 5.0);
        a -= &b;
        assert_eq!(a, Vec3(0.0, -1.0, -2.0));
    }

    #[test]
    fn xyz() {
        let a = Vec3(1.0, 2.0, 3.0);
        assert_eq!(a.x(), 1.0);
        assert_eq!(a.y(), 2.0);
        assert_eq!(a.z(), 3.0);
    }
}
