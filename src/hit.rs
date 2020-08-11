use crate::{Ray, Scalar, Vec3};
use crate::material::Material;

use std::ops::Range;

pub trait Hittable {
	fn hit(&self, ray: Ray, t_range: Range<Scalar>) -> Option<(Hit, &dyn Material)>; 
}

pub struct Hit {
	pub point: Vec3,
	pub normal: Vec3,
	pub t: Scalar,
	pub front_face: bool,
}

pub struct Sphere {
	pub center: Vec3,
	pub radius: Scalar,
	pub material: Box<dyn Material>,
}

impl Hittable for Sphere {
	fn hit(&self, ray: Ray, t_range: Range<Scalar>) -> Option<(Hit, &dyn Material)> {
		let oc = ray.origin - self.center;
		let a = ray.direction.norm_squared();
		let half_b = oc.dot(ray.direction);
		let c = oc.norm_squared() - self.radius.powi(2);

		let discriminant = half_b.powi(2) - a * c;

		if discriminant > 0.0 {
			let root = discriminant.sqrt();
			for r in [-root, root].iter() {
				let temp = (- half_b + r) / a;
				if t_range.contains(&temp) {
					let point = ray.at(temp);
					return Some((Hit {
						point,
						normal: -r.signum() * (point - self.center) / self.radius,
						t: temp,
						front_face: *r >= 0.0,
					},
					&*self.material))
				}
			}
		}
		None
	}
}

pub type ObjectList = [Box<dyn Hittable>];

impl Hittable for ObjectList {
	fn hit(&self, ray: Ray, mut t_range: Range<Scalar>) -> Option<(Hit, &dyn Material)> {
		let mut hit = None;
		for object in self.iter() {
			if let Some(h) = object.hit(ray, t_range.clone()) {
				t_range.end = h.0.t; // closest hit so far
				hit = Some(h);
			}
		}
		hit
	}
}
