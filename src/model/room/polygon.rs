pub use super::*;

#[derive(Debug, Clone)]
pub struct Polygon {
    pub v: (Position, Position, Position),
    pub mat: Material,
}
#[derive(Debug, Clone)]
pub struct Material {
    pub absorption_ratio: Float,
    pub specular_ref_ratio: Float,
    pub scattering_ratio: Float,
}
impl Material {
    pub fn from(
        absorption_ratio: Float,
        specular_ref_ratio: Float,
        scattering_ratio: Float,
    ) -> Self {
        Material {
            absorption_ratio,
            specular_ref_ratio,
            scattering_ratio,
        }
    }
    pub fn new() -> Self {
        Material {
            absorption_ratio: 0.2,
            specular_ref_ratio: 0.5,
            scattering_ratio: 0.5,
        }
    }
}
impl Polygon {
    pub fn u(&self) -> Vector {
        self.v.1 - self.v.0
    }
    pub fn v(&self) -> Vector {
        self.v.2 - self.v.0
    }
    pub fn from_vertex(v: (Position, Position, Position), mat: Material) -> Polygon {
        Polygon { v, mat }
    }
    pub fn from_uv(o: Position, u: Vector, v: Vector, mat: Material) -> Polygon {
        Polygon {
            v: (o, o + u, o + v),
            mat,
        }
    }
    pub fn norm(&self) -> Vector {
        let norm = Vector::cross(self.u(), self.v());
        norm / norm.norm2().sqrt()
    }
}
