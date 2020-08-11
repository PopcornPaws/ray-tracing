pub mod camera;
pub mod hit;
pub mod material;
pub mod random;
pub mod ray;
pub mod vec;

pub use camera::Camera;
pub use ray::Ray;
pub use vec::Vec3;

pub type Scalar = f64;
pub const PI: Scalar = std::f64::consts::PI;
