use super::*;
pub use model::*;
#[derive(Debug, Clone)]
pub struct Speaker {
    pub pos: Vector,
    pub direction: Vector,
    pub theta_resolution: u32,
    pub phi_resolution: u32,
    pub phi_max: Float,
}
#[derive(Debug, Clone)]
pub struct Receiver {
    pub pos: Vector,
    pub r: Float,
    pub direction: Vector,
}
impl Speaker {
    pub fn new() -> Self {
        Speaker {
            pos: Vector::new(0.0, 0.0, 0.0),
            direction: Vector::new(1.0, 0.0, 0.0),
            theta_resolution: 10,
            phi_resolution: 10,
            phi_max: std::f64::consts::PI * 0.5,
        }
    }
}
impl Receiver {
    pub fn new() -> Self {
        Receiver {
            pos: Vector::new(0.0, 0.0, 0.0),
            r: 1.0 / 1000.0, // ～ 1cm (default)
            direction: Vector::new(1.0, 0.0, 0.0),
        }
    }
}
