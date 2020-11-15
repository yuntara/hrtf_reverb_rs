pub use super::*;
pub use errors::*;
pub use lib::ray_nodes::*;
pub use ndarray::*;
pub use netcdf::*;

#[derive(Debug)]
pub struct SofaFIR {
    file: File,
    /*data: (Vec<Vec<Float>>, Vec<Vec<Float>>),
    sampling_rate: u32,
    listener_pos: Vector,
    listener_up: Vector,
    listener_view: Vector,
    receiver_pos: (Vector, Vector),
    source_pos: Vec<SourcePosition>,*/
}
#[derive(Debug)]
pub struct SourcePosition {
    elev: Float,
    azim: Float,
    dist: Float,
}
impl SofaFIR {
    pub fn new(path: &str) -> Self {
        let file = netcdf::open(path);
        if file.is_err() {
            panic!("not found");
        }
        Self {
            file: file.unwrap(),
        }
    }
    pub fn sampling_rate(&self) -> Option<Float> {
        if let Some(rate_variable) = self.file.variable("Data.SamplingRate") {
            let value = rate_variable.value::<Float>(None);
            if value.is_ok() {
                return Some(value.unwrap());
            }
        }
        None
    }
    pub fn source_positions(&self) -> Option<Vec<SourcePosition>> {
        let pos_variable = self.file.variable("SourcePosition")?;
        if let Ok(values) = pos_variable.values::<Float>(None, None) {
            let shape = values.shape();
            if shape.len() != 2 {
                return None;
            }
            let mut result = vec![];
            for row in values.axis_iter(Axis(0)) {
                let azim = row.get(IxDyn(&[0]))?.clone();
                let elev = row.get(IxDyn(&[1]))?.clone();
                let dist = row.get(IxDyn(&[2]))?.clone();
                result.push(SourcePosition { azim, elev, dist })
            }
            return Some(result);
        }
        None
    }
    pub fn receiver_positions(&self) -> Option<(Position, Position)> {
        let pos_variable = self.file.variable("ReceiverPosition")?;
        if let Ok(values) = pos_variable.values::<Float>(None, None) {
            if values.shape() != &[2, 3, 1] {
                assert_eq!(values.shape(), &[1, 2, 3]);
            }
            let array = values.into_shape([2, 3, 1]);
            if array.is_err() {
                return None;
            }
            let array = array.unwrap();
            return Some((
                Position::new(
                    array.get((0, 0, 0)).unwrap().clone(),
                    array.get((0, 1, 0)).unwrap().clone(),
                    array.get((0, 2, 0)).unwrap().clone(),
                ),
                (Position::new(
                    array.get((1, 0, 0)).unwrap().clone(),
                    array.get((1, 1, 0)).unwrap().clone(),
                    array.get((1, 2, 0)).unwrap().clone(),
                )),
            ));
        }
        None
    }
    pub fn data_type(&self) -> Option<String> {
        let data_type = self.file.attribute("DataType");
        if data_type.is_none() {
            return None;
        }
        let value = data_type.as_ref().unwrap().value();
        if value.is_err() {
            return None;
        }

        if let AttrValue::Str(type_str) = value.unwrap() {
            return Some(type_str);
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sofa_load() {
        let sofa = SofaFIR::new("./fixtures/fixture.sofa");
        assert_eq!(sofa.sampling_rate(), Some(48000.0));
        assert_eq!(
            sofa.receiver_positions(),
            Some((
                Position::new(0.0, -0.09, 0.0),
                Position::new(0.0, 0.09, 0.0)
            ))
        );
        let source_positions = sofa.source_positions().unwrap();
        assert_eq!(source_positions.len(), 865);
    }
}
