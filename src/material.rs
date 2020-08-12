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
        let cos_theta = -ray.direction.normalized().dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();


        let refracted_direction = if eta * sin_theta > 1.0 || schlick(cos_theta, eta) > rand::random::<Scalar>() {
            // must reflect
            ray.direction.normalized().reflect(hit.normal)
        } else {
            // can refract
            let r_perpendicular = eta * (ray.direction + cos_theta * hit.normal);
            let r_parallel = -(1.0 - r_perpendicular.norm_squared()).abs().sqrt() * hit.normal;

            (r_perpendicular + r_parallel).normalized()
        };

        Scatter {
            attenuation: Vec3::ones(),
            ray: Ray {
                origin: hit.point,
                direction: refracted_direction,
            }
        }
    }
}

fn schlick(cosine: Scalar, eta: Scalar) -> Scalar {
    let r_0 = ((1.0 - eta) / (1.0 + eta)).powi(2);
    r_0 + (1.0 - r_0) * (1.0 - cosine).powi(5)
}
