pub mod camera;
pub mod hit;
pub mod material;
pub mod random;
pub mod ray;
pub mod vec;

pub use camera::Camera;
pub use ray::Ray;
pub use vec::Vec3;

pub type Scalar = f64;
pub const PI: Scalar = std::f64::consts::PI;

use hit::{Hittable, ObjectList, Sphere};
use material::{Dielectric, Lambertian, Metal};
use rand::Rng;

#[rustfmt::skip]
pub fn random_scene() -> Vec<Box<dyn Hittable>> {
    let mut rng = rand::thread_rng();
    let mut scene = Vec::<Box<dyn Hittable>>::new();
    let ground_material = Lambertian { albedo: 0.5 * Vec3::ones() };
    scene.push(Box::new(Sphere {
        center: -1e3 * Vec3::y(),
        radius: 1e3,
        material: Box::new(ground_material),
    }));

    for a in -11..11 {
        for b in -11..11 {
            let random_material: Scalar = rng.gen();

            let center = Vec3(a as Scalar + 0.9 * rng.gen::<Scalar>(), 0.2, b as Scalar + 0.9 * rng.gen::<Scalar>());
            let radius = 0.2;

            if random_material < 0.8 {
                // diffuse
                let albedo = random::vec3();
                scene.push(Box::new(Sphere {
                    center,
                    radius,
                    material: Box::new(Lambertian { albedo }),
                }));
            } else if random_material < 0.95 {
                // metal
                let albedo = 0.5 * random::vec3() + 0.5 * Vec3::ones();
                let fuzz = 0.5 * rng.gen::<Scalar>();
                scene.push(Box::new(Sphere {
                    center,
                    radius,
                    material: Box::new(Metal { albedo, fuzz }),
                }));
            } else {
                // dielectric
                scene.push(Box::new(Sphere {
                    center,
                    radius,
                    material: Box::new(Dielectric { refractive_index: 1.5 }),
                }));
            }
        }
    }

    // add three big spheres
    let radius: Scalar = 1.0;

    scene.push(Box::new(Sphere {
        center: Vec3::y(),
        radius,
        material: Box::new(Dielectric { refractive_index: 1.5 }),
    }));

    scene.push(Box::new(Sphere {
        center: Vec3(-4.0, 1.0, 0.0),
        radius,
        material: Box::new(Lambertian { albedo: Vec3(0.4, 0.2, 0.1) }),
    }));

    scene.push(Box::new(Sphere {
        center: Vec3(4.0, 1.0, 0.0),
        radius,
        material: Box::new(Metal { albedo: Vec3(0.7, 0.6, 0.5), fuzz: 0.0 }),
    }));

    scene
}

pub fn ray_color(r: Ray, scene: &ObjectList, depth: usize) -> Vec3 {
    if let Some((hit, material)) = scene.hit(r, 1e-3..Scalar::MAX) {
        if depth > 0 {
            let scattered = material.scatter(r, hit);
            return scattered.attenuation * ray_color(scattered.ray, scene, depth - 1);
        } else {
            return Vec3::zeros();
        }
    }

    let unit_direction = r.direction.normalized();
    let t = 0.5 * (unit_direction.1 + 1.0);

    (1.0 - t) * Vec3::ones() + t * Vec3(0.5, 0.7, 1.0)
}
