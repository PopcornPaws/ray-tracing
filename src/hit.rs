use crate::{Ray, Scalar, Vec3};

use std::ops::Range;

pub trait Hittable {
	fn hit(&self, ray: &Ray, t_range: Range<Scalar>) -> Option<Hit>; 
}

pub struct Hit {
	pub point: Vec3,
	pub normal: Vec3,
	pub t: Scalar,
}

pub struct Sphere {
	pub center: Vec3,
	pub radius: Scalar,
}

impl Hittable for Sphere {
	fn hit(&self, ray: &Ray, t_range: Range<Scalar>) -> Option<Hit> {
		let oc = ray.origin - self.center;
		let a = ray.direction.norm_squared();
		let half_b = oc.dot(ray.direction);
		let c = oc.norm_squared() - self.radius.powi(2);

		let discriminant = half_b.powi(2) - a * c;

		if discriminant > 0.0 {
			let root = discriminant.sqrt();

			let temp = (- half_b - root) / a; // -b - sqrt() / a
			if t_range.contains(&temp) {
				let point = ray.at(temp);
				return Some(Hit {
					point,
					normal: (point - self.center) / self.radius, 
					t: temp,
				})
			}
		}
		None
	}
}
