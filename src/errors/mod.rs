pub struct Error {
    pub msg: String,
}
impl Error {
    pub fn new(msg: &str) -> Error {
        Error { msg: msg.into() }
    }
}

impl std::convert::From<netcdf::error::Error> for Error {
    fn from(error: netcdf::error::Error) -> Self {
        match error {
            netcdf::error::Error::Netcdf(_nc_type) => Error::new(""),
            netcdf::error::Error::Str(msg) => Error::new(&msg),
            netcdf::error::Error::IndexLen => Error::new("index len"),
            netcdf::error::Error::SliceLen => Error::new("slice len"),
            netcdf::error::Error::BufferLen(_a, _b) => Error::new("buffer len"),
            netcdf::error::Error::IndexMismatch => Error::new("index mismatch"),
            netcdf::error::Error::SliceMismatch => Error::new("slice mismatch"),
            netcdf::error::Error::ZeroSlice => Error::new("zero slice"),
            netcdf::error::Error::Stride => Error::new("stride"),
            netcdf::error::Error::TypeMismatch => Error::new("type mismatch"),
            netcdf::error::Error::TypeUnknown(_nc_type) => Error::new("type unknown"),
            netcdf::error::Error::AlreadyExists => Error::new("already exists"),
            netcdf::error::Error::NotFound(msg) => Error::new(&msg),
            netcdf::error::Error::Ambiguous => Error::new("ambiguous"),
            netcdf::error::Error::Overflow => Error::new("overflow"),
            netcdf::error::Error::Conversion(_try_from_int_error) => Error::new("tryfrominterror"),
            netcdf::error::Error::WrongDataset => Error::new("wrong dataset"),
            netcdf::error::Error::Utf8Conversion(_from_utf8_error) => {
                Error::new("format utf8 error")
            }
        }
    }
}
