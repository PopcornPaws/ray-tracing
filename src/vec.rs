use crate::Scalar;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub Scalar, pub Scalar, pub Scalar);

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<Scalar> for Vec3 {
    type Output = Self;
    fn mul(self, other: Scalar) -> Self {
        Vec3(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Mul<Vec3> for Scalar {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self * other.0, self * other.1, self * other.2)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Div<Scalar> for Vec3 {
    type Output = Self;
    fn div(self, other: Scalar) -> Self {
        Vec3(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

impl MulAssign<Scalar> for Vec3 {
    fn mul_assign(&mut self, other: Scalar) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

impl DivAssign<Scalar> for Vec3 {
    fn div_assign(&mut self, other: Scalar) {
        self.0 /= other;
        self.1 /= other;
        self.2 /= other;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Vec3 {
    #[inline]
    pub fn ones() -> Self {
        Self(1.0, 1.0, 1.0)
    }

    #[inline]
    pub fn zeros() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn x() -> Self {
        Self(1.0, 0.0, 0.0)
    }

    #[inline]
    pub fn y() -> Self {
        Self(0.0, 1.0, 0.0)
    }

    #[inline]
    pub fn z() -> Self {
        Self(0.0, 0.0, 1.0)
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let v = 2.0 * Self(rng.gen(), rng.gen(), rng.gen()) - Self::ones();
            if v.norm_squared() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Self {
        let v = Self::random_in_unit_sphere();
        if v.dot(normal) > 0.0 {
            v
        } else {
            -v
        }
    }

    pub fn random_unit_vector() -> Self {
        let mut rng = rand::thread_rng();
        let a: Scalar = 2.0 * crate::PI * rng.gen::<Scalar>();
        let z: Scalar = 2.0 * rng.gen::<Scalar>() - 1.0;
        let r: Scalar = (1.0 - z * z).sqrt();
        Self(r * a.cos(), r * a.sin(), z)
    }

    #[inline]
    pub fn cross(&self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    #[inline]
    pub fn dot(&self, other: Self) -> Scalar {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    #[inline]
    pub fn normalized(self) -> Vec3 {
        self / self.norm()
    }

    #[inline]
    pub fn norm_squared(&self) -> Scalar {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    #[inline]
    pub fn norm(&self) -> Scalar {
        self.norm_squared().sqrt()
    }

    #[inline]
    pub fn as_pixel(self) -> [u8; 3] {
        let v = Vec3(self.0.sqrt(), self.1.sqrt(), self.2.sqrt()); // gamma correction
        let pixel = v * 255.999;
        [
            pixel.0.min(255.0) as u8,
            pixel.1.min(255.0) as u8,
            pixel.2.min(255.0) as u8,
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_norm() {
        let v = Vec3(3.0, 4.0, 5.0);
        assert!((v.norm_squared() - 50.0).abs() < 1e-9);

        let u = dbg!(v.normalized());
        assert!((u.0 - v.0 / v.norm()).abs() < 1e-9);
        assert!((u.1 - v.1 / v.norm()).abs() < 1e-9);
        assert!((u.2 - v.2 / v.norm()).abs() < 1e-9);
    }

    #[test]
    fn test_dot() {
        let v = Vec3(3.0, 4.0, 5.0);
        assert!((v.norm_squared() - v.dot(v)).abs() < 1e-9);
    }

    #[test]
    fn test_neg() {
        let v = Vec3::ones();
        let vn = -v;
        assert!((v + vn).norm() < 1e-9);
    }

    #[test]
    fn test_random_in_unit_circle() {
        let v = Vec3::random_in_unit_circle();
        assert!(v.norm() < 1.0);
    }
}
