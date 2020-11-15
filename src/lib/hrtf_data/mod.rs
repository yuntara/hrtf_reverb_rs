pub mod sofa_fir;

pub use super::*;
pub use lib::ray_nodes::*;
pub use sofa_fir::*;

#[derive(Debug, Clone)]
pub struct SourcePosition {
    theta: Float,
    phi: Float,
    dist: Float,
}

#[derive(Debug, Clone)]
pub struct HRTFData {
    data: Vec<Vec<Float>>,
    pub range_x: (u32, u32),
    pub range_y: (u32, u32),
    //sampling_rate: u32,
    //receiver_pos: (Vector, Vector),
}
impl HRTFData {
    pub fn new() -> Self {
        Self {
            data: vec![vec![]],
            range_x: (0, 0),
            range_y: (0, 0),
        }
    }
    pub fn adjust(&self, recv: ReceiveEvent) -> Option<AdjustResult> {
        Some(AdjustResult {
            data: 0,
            intensity: 1.0,
            delay: 2.0,
        })
    }
}
