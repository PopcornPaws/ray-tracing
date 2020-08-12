use std::fs::File;
use std::io::Write;

use rand::Rng;

use ray_tracing::hit::{Hittable, Sphere};
use ray_tracing::material::{Dielectric, Lambertian, Metal};
use ray_tracing::{Camera, Scalar, Vec3};

use ray_tracing::ray_color;

fn do_main() -> std::io::Result<()> {
    // image
    let aspect_ratio: Scalar = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height = (image_width as Scalar / aspect_ratio) as usize;

    // scene
    let mut scene = Vec::<Box<dyn Hittable>>::with_capacity(2);

    // center sphere
    scene.push(Box::new(Sphere {
        center: -Vec3::z(),
        radius: 0.5,
        material: Box::new(Lambertian {
            albedo: Vec3(0.7, 0.3, 0.3),
        }),
    }));

    // left sphere
    scene.push(Box::new(Sphere {
        center: Vec3(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(Metal {
            albedo: Vec3(0.8, 0.8, 0.8),
            fuzz: 1.0,
        }),
    }));

    // right sphere
    scene.push(Box::new(Sphere {
        center: Vec3(1.1, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(Dielectric {
            refractive_index: 1.5,
        }),
    }));

    // ground
    scene.push(Box::new(Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Box::new(Lambertian {
            albedo: Vec3(0.8, 0.8, 0.0),
        }),
    }));

    // camera
    let look_from = Vec3(3.0, 3.0, 2.0);
    let look_at = -Vec3::z();
    let up_vector = Vec3::y();
    let vfov: Scalar = 20.0;
    let aperture: Scalar = 2.0;
    let dist_to_focus: Scalar = (look_from - look_at).norm();
    let camera = Camera::new(
        look_from,
        look_at,
        up_vector,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let depth: usize = 50;
    let samples_per_pixel: usize = 50;
    let mut rng = rand::thread_rng();

    let mut file = File::create("assets/rays.ppm")?;
    file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;

    for h in (0..image_height).rev() {
        print!("Scanlines remaining: {}\r", h);
        for w in 0..image_width {
            let mut pixel_color = Vec3::zeros();
            for _ in 0..samples_per_pixel {
                let u = (w as Scalar + rng.gen::<Scalar>()) / (image_width - 1) as Scalar;
                let v = (h as Scalar + rng.gen::<Scalar>()) / (image_height - 1) as Scalar;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(ray, scene.as_slice(), depth);
            }

            let pixel = (pixel_color / samples_per_pixel as Scalar).as_pixel();
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
