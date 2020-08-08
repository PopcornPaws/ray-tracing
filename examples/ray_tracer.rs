use std::fs::File;
use std::io::Write;
use std::ops::Range;

use rand::Rng;

use ray_tracing::hit::{Hittable, ObjectList, Sphere};
use ray_tracing::{Camera, Ray, Scalar, Vec3};

fn ray_color(r: &mut Ray, world: &ObjectList, mut depth: usize) -> Vec3 {
    while let Some(hit) = world.hit(
        &r,
        Range {
            start: 0.0,
            end: Scalar::MAX,
        },
    ) {
        if depth == 0 {
            return Vec3::zeros();
        }
        depth -= 1;
        let target = hit.point + hit.normal + Vec3::random_in_unit_circle();
        *r = Ray {
            origin: hit.point,
            direction: target - hit.point,
        };
        //return 0.5 * (hit.normal + Vec3::ones());
    }

    let unit_direction = r.direction.normalized();
    let t = 0.5 * (unit_direction.1 + 1.0);

    0.5 * ((1.0 - t) * Vec3::ones() + t * Vec3(0.5, 0.7, 1.0))
}

fn do_main() -> std::io::Result<()> {
    // image
    let aspect_ratio: Scalar = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height = (image_width as Scalar / aspect_ratio) as usize;

    // world
    let mut world = Vec::<Box<dyn Hittable>>::with_capacity(2);
    world.push(Box::new(Sphere {
        center: -Vec3::z(),
        radius: 0.5,
    }));
    world.push(Box::new(Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    // camera
    let viewport_height: Scalar = 2.0;
    let focal_length: Scalar = 1.0;
    let camera = Camera::new(aspect_ratio, viewport_height, focal_length);

    let depth: usize = 50;
    let samples_per_pixel: usize = 100;
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
                let mut ray = camera.get_ray(u, v);
                pixel_color += ray_color(&mut ray, world.as_slice(), depth);
            }

            let pixel = (pixel_color / samples_per_pixel as Scalar).as_pixel();
            file.write_all(format!("{} {} {}\n", pixel[0].min(255), pixel[1].min(255), pixel[2].min(255)).as_bytes())?;
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
