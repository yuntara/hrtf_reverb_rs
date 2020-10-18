pub use super::*;

pub use ray::*;

#[derive(Debug, Clone)]
pub struct Room {
    pub planes: Vec<Polygon>,
}

impl Room {
    pub fn new() -> Room {
        Room { planes: vec![] }
    }
    pub fn intersect(r: &Ray, p: &Polygon) -> Option<Position> {
        // p.v[0] + s * u + t * v = r.o + r.norm * p;
        //  s * u + t * v - p * r.norm= r.o -p.v[0];
        let r_inv = -r.norm;
        let det = Matrix::det(p.u(), p.v(), r_inv);
        if det <= 0.0 {
            return None;
        }
        let d = r.o - p.v.0;
        let u = Matrix::det(d, p.v(), r_inv) / det;
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let v = Matrix::det(d, p.u(), r_inv) / det;
        if v < 0.0 || v > 1.0 {
            return None;
        }
        let t = Matrix::det(p.u(), p.v(), d) / det;
        if t < 0.0 {
            return None;
        }
        Some(r.o + r.norm.clone() * t)
    }
    pub fn refrect(r: &Ray, p: &Polygon) -> Vector {
        let p_norm = p.norm();
        let a = -Vector::dot(r.norm, p_norm);
        r.norm + p_norm * (2.0 * a)
    }
    pub fn find_intersection(&self, r: &Ray) -> Option<(Polygon, Position)> {
        let mut min_dist = None;
        let mut result = None;
        for poly in self.planes.iter() {
            let intersection = Room::intersect(r, poly);

            if let Some(intr) = intersection {
                let dist = (intr - r.o).norm2();
                if min_dist.map_or(true, |md| dist < md) {
                    min_dist = Some(dist);
                    result = Some((poly.clone(), intr));
                }
            }
        }
        result
    }
    pub fn find_first_intersection(&self, r: &Ray) -> Option<(&Polygon, Position)> {
        for poly in self.planes.iter() {
            let intersection = Room::intersect(r, poly);

            if let Some(intr) = intersection {
                return Some((poly, intr));
            }
        }
        None
    }
}
