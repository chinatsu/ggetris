use std::ops;


/// A simple point struct to use for matrix cells and such
#[derive(Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point {
            x: x,
            y: y,
        }
    }
 }

/// Functionality to directly add and subtract Points
impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, _rhs: Point) -> Point {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, _rhs: Point) -> Point {
        Point {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y
        }
    }
}
