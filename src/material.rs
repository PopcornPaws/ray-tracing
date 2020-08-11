use crate::{Ray, Scalar, Vec3};
use crate::hit::Hit;
use crate::random;

pub trait Material {
    fn scatter(&self, ray: Ray, hit: Hit) -> Scatter;
}

pub struct Scatter {
    pub attenuation: Vec3,
    pub ray: Ray,
}

pub struct Lambertian {
    pub albedo: Vec3,
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: Scalar,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, hit: Hit) -> Scatter {
        Scatter {
            attenuation: self.albedo,
            ray: Ray {
                origin: hit.point,
                direction: (hit.normal + random::unit_vec3()).normalized(),
            }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit: Hit) -> Scatter {
        Scatter {
            attenuation: self.albedo,
            ray: Ray {
                origin: hit.point,
                direction: (ray.direction.reflect(hit.normal) + self.fuzz * random::vec3_in_unit_sphere()).normalized(),
            }
        }
    }
}
