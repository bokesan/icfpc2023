use crate::geometry::{Point, Vector};

fn dot(a: Vector<f64>, b: Vector<f64>) -> f64 {
    a.x * b.x + a.y * b.y
}

pub fn line_circle_intersect(e: Point<f64>, l: Point<f64>, c: Point<f64>, r: f64) -> bool {
    let d = l - e;
    let f = e - c;
    let a = dot(d, d);
    let b = 2.0 * dot(f, d);
    let c1 = dot(f, f) - r * r;
    let mut disc = b * b - 4.0 * a * c1;
    if disc <= 0.0 {
        false
    } else {
        disc = disc.sqrt();
        let t1 = (-b - disc) / (2.0 * a);
        let t2 = (-b + disc) / (2.0 * a);
        (t1 >= 0.0 && t1 <= 1.0) || (t2 >= 0.0 && t2 <= 1.0)
    }
}

fn sq(x: f64) -> f64 {
    x * x
}

fn touch(a: Point<f64>, b: Point<f64>, c: Point<f64>, r: f64) -> bool {
    // compute the euclidean distance between A and B
    let LAB = ( sq(b.x-a.x) + sq(b.y-a.y) ).sqrt();

// compute the direction vector D from A to B
    let Dx = (b.x-a.x)/LAB;
    let Dy = (b.y-a.y)/LAB;

// the equation of the line AB is x = Dx*t + Ax, y = Dy*t + Ay with 0 <= t <= LAB.

// compute the distance between the points A and E, where
// E is the point of AB closest the circle center (Cx, Cy)
    let t = Dx*(c.x-a.x) + Dy*(c.y-a.y);

// compute the coordinates of the point E
    let Ex = t*Dx+a.x;
    let Ey = t*Dy+a.y;

// compute the euclidean distance between E and C
    let LEC = (sq(Ex-c.x) + sq(Ey-c.y)).sqrt();

// test if the line intersects the circle
    LEC == r
}