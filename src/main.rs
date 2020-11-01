pub mod errors;
pub mod lib;
pub mod model;
pub use lib::*;
fn main() {
    println!("Hello, world!");
    let hrtf = HRTFData::new();
    Reverbrator::simulate(&hrtf, Options::default());
}
