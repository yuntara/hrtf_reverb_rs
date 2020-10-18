pub use super::*;
pub use lib::ray_nodes::*;

#[derive(Debug, Clone)]
pub struct HRTFRawData {
    data: (Vec<Float>,Vec<Float>),
    sampling_rate: u32,
    listener_pos: Vector,
    listener_up: Vector,
    listener_view: Vector,
    receiver_pos: (Vector, Vector),
    source_pos:Vec<SourcePosition>
}
#[derive(Debug, Clone)]
pub struct SourcePosition{
    theta:Float,
    phi:Float,
    dist:Float,
}

#[derive(Debug, Clone)]
pub struct HRTFData {
    data: Vec<Float>,
    sampling_rate: u32,
    listener_pos: Vector,
    listener_up: Vector,
    listener_view: Vector,
    receiver_pos: (Vector, Vector),
    source_pos:
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
