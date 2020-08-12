use crate::random;
use crate::{Ray, Scalar, Vec3, PI};

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: Scalar,
}

impl Camera {
    #[inline]
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        up_vector: Vec3,
        vfov: Scalar,
        aspect_ratio: Scalar,
        aperture: Scalar,
        focus_dist: Scalar,
    ) -> Self {
        let viewport_height = 2.0 * (vfov * PI / 360.0).tan();
        let viewport_width = viewport_height * aspect_ratio;

        let w = (look_from - look_at).normalized();
        let u = up_vector.cross(w).normalized();
        let v = w.cross(u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let origin = look_from;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    #[inline]
    pub fn get_ray(&self, s: Scalar, t: Scalar) -> Ray {
        let rd = self.lens_radius * random::vec3_in_unit_disc();
        let offset = self.u * rd.0 + self.v * rd.1;
        Ray {
            origin: self.origin + offset,
            direction: (self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset)
                .normalized(),
        }
    }
}
