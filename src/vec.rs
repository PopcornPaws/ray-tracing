use crate::Scalar;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy)]
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
		Vec3(self.0 * other, self.1 * other, self.2 * other)
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

impl Vec3 {
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
}
