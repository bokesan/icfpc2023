use std::fmt;
use std::ops;
use serde::{Deserialize, Serialize};


// This code for Point and Vector has been snarfed from
// https://github.com/redox-os/rusttype/blob/master/src/geometry.rs


/// A point in 2-dimensional space, with each dimension of type `N`.
///
/// Legal operations on points are addition and subtraction by vectors, and
/// subtraction between points, to give a vector representing the offset between
/// the two points. Combined with the legal operations on vectors, meaningful
/// manipulations of vectors and points can be performed.
///
/// For example, to interpolate between two points by a factor `t`:
///
/// ```
/// # use rusttype::*;
/// # let t = 0.5; let p0 = point(0.0, 0.0); let p1 = point(0.0, 0.0);
/// let interpolated_point = p0 + (p1 - p0) * t;
/// ```
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point<N> {
    pub x: N,
    pub y: N,
}

/// A vector in 2-dimensional space, with each dimension of type `N`.
///
/// Legal operations on vectors are addition and subtraction by vectors,
/// addition by points (to give points), and multiplication and division by
/// scalars.
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Vector<N> {
    pub x: N,
    pub y: N,
}

/// A convenience function for generating `Point`s.
#[inline]
pub fn point<N>(x: N, y: N) -> Point<N> {
    Point { x, y }
}

/// A convenience function for generating `Vector`s.
#[inline]
pub fn vector<N>(x: N, y: N) -> Vector<N> {
    Vector { x, y }
}

impl Point<f64> {

    pub fn distance(&self, p: &Point<f64>) -> f64 {
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        (dx*dx + dy*dy).sqrt()
    }

}

impl<N: ops::Sub<Output = N>> ops::Sub for Point<N> {
    type Output = Vector<N>;
    fn sub(self, rhs: Point<N>) -> Vector<N> {
        vector(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<N: ops::Add<Output = N>> ops::Add for Vector<N> {
    type Output = Vector<N>;
    fn add(self, rhs: Vector<N>) -> Vector<N> {
        vector(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<N: ops::Sub<Output = N>> ops::Sub for Vector<N> {
    type Output = Vector<N>;
    fn sub(self, rhs: Vector<N>) -> Vector<N> {
        vector(self.x - rhs.x, self.y - rhs.y)
    }
}

#[allow(dead_code)]
impl<T: ops::Mul<Output = T> + Copy> Vector<T> {

    /// Scale vector by constant
    pub fn scale(self, c: T) -> Self {
        vector(self.x * c, self.y * c)
    }

}

impl<N: ops::Add<Output = N>> ops::Add<Vector<N>> for Point<N> {
    type Output = Point<N>;
    fn add(self, rhs: Vector<N>) -> Point<N> {
        point(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<N: ops::Sub<Output = N>> ops::Sub<Vector<N>> for Point<N> {
    type Output = Point<N>;
    fn sub(self, rhs: Vector<N>) -> Point<N> {
        point(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<N: ops::Add<Output = N>> ops::Add<Point<N>> for Vector<N> {
    type Output = Point<N>;
    fn add(self, rhs: Point<N>) -> Point<N> {
        point(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: fmt::Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl<T: fmt::Display> fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}|{})", self.x, self.y)
    }
}

impl<T: Copy + ops::Add<Output = T> + ops::Mul<Output = T> + Into<f64>> Vector<T> {

    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f64 {
        self.length_squared().into().sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_add() {
        let v1: Vector<i32> = vector(2, 3);
        let v2: Vector<i32> = vector(6, 9);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 8);
        assert_eq!(v3.y, 12);
        assert_eq!(v3, vector(8, 12));
    }

    #[test]
    fn test_point_vector_add() {
        let p1: Point<i32> = point(2, 3);
        let v: Vector<i32> = vector(6, 9);
        let p2 = p1 + v;
        assert_eq!(p2.x, 8);
        assert_eq!(p2.y, 12);
        assert_eq!(p2, point(8, 12));
    }

    #[test]
    fn test_point_subtract() {
        let p1: Point<i32> = point(2, 3);
        let p2: Point<i32> = point(10, 10);
        let v: Vector<i32> = p2 - p1;
        assert_eq!(v.x, 8);
        assert_eq!(v.y, 7);
    }

    #[test]
    fn test_vector_scale() {
        let v: Vector<i32> = vector(3, 4);
        let expected = vector(9, 12);
        assert_eq!(v.scale(3), expected);
    }

    #[test]
    fn test_vector_length() {
        const DELTA: f64 = 0.00000001;
        let v = vector(1,1);
        let expected: f64 = (2.0 as f64).sqrt();
        assert!((v.length() - expected).abs() < DELTA);
    }
}