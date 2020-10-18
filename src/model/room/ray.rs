pub use super::*;

#[derive(Debug, Clone)]
pub struct Ray {
    pub o: Position,
    pub norm: Vector,
}
#[derive(Debug, Clone)]
pub struct RayWithAttributes {
    pub ray: Ray,
    pub dist: Float,
    pub intensity: Float,
}
