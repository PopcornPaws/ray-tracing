use std::fs::File;
use std::io::Write;

use rand::Rng;

use ray_tracing::random_scene;
use ray_tracing::ray_color;
use ray_tracing::{Camera, Scalar, Vec3};

fn do_main() -> std::io::Result<()> {
    // image
    let aspect_ratio: Scalar = 3.0 / 2.0;
    let image_width: usize = 1200;
    let image_height = (image_width as Scalar / aspect_ratio) as usize;
    // scene
    let scene = random_scene();
    // camera
    let look_from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3::zeros();
    let up_vector = Vec3::y();
    let vfov: Scalar = 20.0;
    let aperture: Scalar = 0.1;
    let dist_to_focus: Scalar = 10.0;
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
