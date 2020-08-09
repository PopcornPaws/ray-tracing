use crate::{Scalar, Vec3};
use rand::Rng;

pub fn lazy_lambertian() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let v = 2.0 * Vec3(rng.gen(), rng.gen(), rng.gen()) - Vec3::ones();
        if v.norm_squared() < 1.0 {
            return v;
        }
    }
}

pub fn true_lambertian() -> Vec3 {
    let mut rng = rand::thread_rng();
    let a: Scalar = 2.0 * crate::PI * rng.gen::<Scalar>();
    let z: Scalar = 2.0 * rng.gen::<Scalar>() - 1.0;
    let r: Scalar = (1.0 - z * z).sqrt();
    Vec3(r * a.cos(), r * a.sin(), z)
}

pub fn hemispherical(normal: Vec3) -> Vec3 {
    let v = lazy_lambertian();
    if v.dot(normal) > 0.0 {
        v
    } else {
        -v
    }
}