pub mod polygon;
pub mod ray;
pub mod room;

pub use super::*;
pub use polygon::*;
pub use ray::*;
pub use room::*;

#[derive(Debug)]
struct Matrix {}
impl Matrix {
    fn det(a: Vector, b: Vector, c: Vector) -> Float {
        a.x * b.y * c.z + a.y * b.z * c.x + a.z * b.x * c.y
            - a.x * b.z * c.y
            - a.y * b.x * c.z
            - a.z * b.y * c.x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr, $eps:expr) => {{
            let a = $a;
            let b = $b;
            let c = a - b;
            let eps = $eps;
            assert!(
                c.x.abs() < eps,
                "assertion failed: `(left !== right)` \
            (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
                a,
                b,
                eps,
                c
            );
            assert!(
                c.y.abs() < eps,
                "assertion failed: `(left !== right)` \
            (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
                a,
                b,
                eps,
                c
            );
            assert!(
                c.z.abs() < eps,
                "assertion failed: `(left !== right)` \
            (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
                a,
                b,
                eps,
                c
            );
        }};
    }
    #[test]
    fn can_calc_intersect() {
        let poly_a = Polygon::from_vertex(
            (
                Vector::new(0.0, 0.0, 0.0),
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
            ),
            Material::new(),
        );
        let ray = Ray {
            o: Vector::new(0.0, 0.0, 1.0),
            norm: Vector::new(0.0, 0.0, -1.0),
        };
        let intersection_a = Room::intersect(&ray, &poly_a).unwrap();
        assert_approx_eq!(intersection_a, Vector::new(0.0, 0.0, 0.0), 1e-10);
        let ref_a = Room::refrect(&ray, &poly_a);
        assert_approx_eq!(ref_a, Vector::new(0.0, 0.0, 1.0), 1e-10);
    }
}
