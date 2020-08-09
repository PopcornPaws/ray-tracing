use crate::{Ray, Vec3};
use crate::hit::Hit;
use crate::random;

pub trait Material {
    fn scatter(&self, ray: Ray, hit: Hit) -> Scatter;
}

pub struct Scatter {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub struct Lambertian {
    albedo: Vec3,
}

pub struct Metal {
    albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, hit: Hit) -> Scatter {
        Scatter {
            attenuation: self.albedo,
            scattered: Ray {
                origin: hit.point,
                direction: hit.normal + random::random_unit_vector(),
            }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit: Hit) -> Scatter {
        Scatter {
            attenuation: self.albedo,
            scattered: Ray {
                origin: hit.point,
                direction: ray.direction.normalized().reflect(hit.normal)
            }
        }
    }
}

