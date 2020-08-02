use std::fs::File;
use std::io::Write;

use ray_tracing::{Ray, Scalar, Vec3};

fn is_sphere_hit(center: Vec3, radius: Scalar, ray: &Ray) -> Scalar {
	let oc = ray.origin - center;
	let a: Scalar = ray.direction.norm_squared();
	let half_b: Scalar = ray.direction.dot(oc);
	let c: Scalar = oc.norm_squared() - radius.powi(2);

	let discriminant = half_b.powi(2) - a * c;

	if discriminant < 0.0 {
		return -1.0
	} else {
		return (-half_b - discriminant.sqrt()) / a
	}
}

fn ray_color(r: &Ray) -> Vec3 {
	let mut t = is_sphere_hit(Vec3(0.0, 0.0, -1.0), 0.5, r);
	if t > 0.0 {
		let normal = (r.at(t) - Vec3(0.0, 0.0, -1.0)).normalized();
		return 0.5 * Vec3(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0)
	} 

	let unit_direction = r.direction.normalized();
	t = 0.5 * (unit_direction.y() + 1.0);

	(1.0 - t) * Vec3::ones() + t * Vec3(0.5, 0.7, 1.0)
}

fn do_main() -> std::io::Result<()> {
	// image
	let aspect_ratio: Scalar = 16.0 / 9.0;
	let image_width: usize = 400;
	let image_height = (image_width as Scalar / aspect_ratio) as usize;

	// camera 
	let viewport_height: Scalar = 2.0;
	let viewport_width: Scalar = viewport_height * aspect_ratio;
	let focal_length: Scalar = 1.0;

	let origin = Vec3(0.0, 0.0, 0.0);
	let horizontal = Vec3(viewport_width, 0.0, 0.0);
	let vertical = Vec3(0.0, viewport_height, 0.0);
	let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);

	let mut file = File::create("assets/rays.ppm")?;
	file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;

	for h in (0..image_height).rev() {
		print!("Scanlines remaining: {}\r", h);
		for w in 0..image_width {
			let a = w as Scalar / (image_width - 1) as Scalar;
			let b = h as Scalar / (image_height - 1) as Scalar;

			let ray = Ray {
				origin,
				direction: lower_left_corner + a * horizontal + b * vertical - origin,
			};

			let pixel = ray_color(&ray).as_pixel();

			file.write_all(format!("{} {} {}\n", pixel[0], pixel[1], pixel[2]).as_bytes())?;
		}
	}
	Ok(())
}

fn main() {
	match do_main() {
		Ok(()) => println!("Successfully written .ppm file!"),
		Err(e) => println!("{}", e),
	}
}
