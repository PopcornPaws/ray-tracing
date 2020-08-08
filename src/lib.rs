pub mod camera;
pub mod hit;
pub mod ray;
pub mod vec;

pub use camera::Camera;
pub use vec::Vec3;
pub use ray::Ray;

pub type Scalar = f64;
pub const PI: Scalar = std::f64::consts::PI;
