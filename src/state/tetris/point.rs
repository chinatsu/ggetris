use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize
}

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
