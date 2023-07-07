use crate::geometry::{Point, Vector};

fn dot(a: Vector<f64>, b: Vector<f64>) -> f64 {
    a.x * b.x + a.y * b.y
}

pub fn line_circle_intersect(e: Point<f64>, l: Point<f64>, c: Point<f64>, r: f64) -> bool {
    let d = l - e;
    let f = e - c;
    let a = dot(d, d);
    let b = 2.0 * dot(f, d);
    let c = dot(f, f) - r * r;
    let mut disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        false
    } else {
        disc = disc.sqrt();
        let t1 = (-b - disc) / (2.0 * a);
        let t2 = (-b + disc) / (2.0 * a);
        (t1 >= 0.0 && t1 <= 1.0) || (t2 >= 0.0 && t2 <= 1.0)
    }

}