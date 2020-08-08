use crate::{Ray, Scalar, Vec3};

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    #[inline]
    pub fn new(aspect_ratio: Scalar, viewport_height: Scalar, focal_length: Scalar) -> Self {
        let viewport_width = viewport_height * aspect_ratio;
        let horizontal = viewport_width * Vec3::x();
        let vertical = viewport_height * Vec3::y();
        let origin = Vec3::zeros();
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - focal_length * Vec3::z(),//(0.0, 0.0, focal_length),
        }
    }

    #[inline]
    pub fn get_ray(&self, u: Scalar, v: Scalar) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}
