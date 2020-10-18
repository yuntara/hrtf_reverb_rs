pub use super::*;
pub use errors::*;
pub use lib::ray_nodes::*;
pub use netcdf::*;
#[derive(Debug, Clone)]
pub struct HRTFRawData {
    data: (Vec<Vec<Float>>, Vec<Vec<Float>>),
    sampling_rate: u32,
    listener_pos: Vector,
    listener_up: Vector,
    listener_view: Vector,
    receiver_pos: (Vector, Vector),
    source_pos: Vec<SourcePosition>,
}

impl HRTFRawData {
    pub fn from_sofa(path: &str) -> Result<(), errors::Error> {
        let file = netcdf::open(path)?;
        let data_type = &file.attribute("DataType");
        if data_type.is_none() {
            return Err(Error::new("not found data type"));
        }
        if let AttrValue::Str(type_str) = data_type.as_ref().unwrap().value()? {
            if type_str != "FIR" {
                return Err(Error::new("data type must be FIR"));
            }
        }

        Ok(())
    }
}
