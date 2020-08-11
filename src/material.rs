use crate::hit::Hit;
use crate::random;
use crate::{Ray, Scalar, Vec3};

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

pub struct Dielectric {
    pub refractive_index: Scalar,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, hit: Hit) -> Scatter {
        Scatter {
            attenuation: self.albedo,
            ray: Ray {
                origin: hit.point,
                direction: (hit.normal + random::unit_vec3()).normalized(),
            },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit: Hit) -> Scatter {
        Scatter {
            attenuation: self.albedo,
            ray: Ray {
                origin: hit.point,
                direction: (ray.direction.reflect(hit.normal)
                    + self.fuzz * random::vec3_in_unit_sphere())
                .normalized(),
            },
        }
    }
}

impl Material for Dielectric {
    #[rustfmt::skip]
    fn scatter(&self, ray: Ray, hit: Hit) -> Scatter {
        let eta = if hit.front_face { 1.0 / self.refractive_index } else { self.refractive_index };
        let refracted = refract(ray.direction.normalized(), hit.normal, eta);

        Scatter {
            attenuation: Vec3::ones(),
            ray: Ray {
                origin: hit.point,
                direction: refracted,
            }
        }
    }
}

fn refract(incoming: Vec3, normal: Vec3, eta: Scalar) -> Vec3 {
    let cos_theta = -incoming.dot(normal);
    let r_perpendicular = eta * (incoming + cos_theta * normal);
    let r_parallel = -(1.0 - r_perpendicular.norm_squared()).abs().sqrt() * normal;

    r_perpendicular + r_parallel
}
