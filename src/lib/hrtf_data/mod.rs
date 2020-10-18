pub mod hrtf_raw;

pub use super::*;
pub use hrtf_raw::*;
pub use lib::ray_nodes::*;

#[derive(Debug, Clone)]
pub struct SourcePosition {
    theta: Float,
    phi: Float,
    dist: Float,
}

#[derive(Debug, Clone)]
pub struct HRTFData {
    data: Vec<Vec<Float>>,
    //sampling_rate: u32,
    //receiver_pos: (Vector, Vector),
}
impl HRTFData {
    pub fn new() -> Self {
        Self { data: vec![vec![]] }
    }
    pub fn adjust(&self, recv: ReceiveEvent) -> Option<AdjustResult> {
        Some(AdjustResult {
            data: 0,
            intensity: 1.0,
            delay: 2.0,
        })
    }
}
