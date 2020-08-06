use std::fs::File;
use std::io::Write;
use std::ops::Range;

use ray_tracing::hit::{Hittable, ObjectList, Sphere};
use ray_tracing::{Ray, Scalar, Vec3};

fn ray_color(r: &Ray, world: &ObjectList) -> Vec3 {
    if let Some(hit) = world.hit(
        r,
        Range {
            start: 0.0,
            end: Scalar::MAX,
        },
    ) {
        return 0.5 * (hit.normal + Vec3(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction.normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Vec3::ones() + t * Vec3(0.5, 0.7, 1.0)
}

fn do_main() -> std::io::Result<()> {
    // image
    let aspect_ratio: Scalar = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height = (image_width as Scalar / aspect_ratio) as usize;

    // world
    let mut world = Vec::<Box<dyn Hittable>>::with_capacity(2);
    world.push(Box::new(Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    world.push(Box::new(Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    // camera
    let viewport_height: Scalar = 2.0;
    let viewport_width: Scalar = viewport_height * aspect_ratio;
    let focal_length: Scalar = 1.0;

    let origin = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);

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

            let pixel = ray_color(&ray, world.as_slice()).as_pixel();

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
