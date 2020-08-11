use crate::{Scalar, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: Scalar) -> Vec3 {
        self.origin + t * self.direction
    }
}
