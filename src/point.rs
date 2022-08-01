use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x: x, y: y, z: z }
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> ops::Mul<T> for Point
where
    f64: From<T>,
    T: Copy,
{
    type Output = Point;

    fn mul(self, other: T) -> Self {
        Point {
            x: self.x * f64::from(other),
            y: self.y * f64::from(other),
            z: self.z * f64::from(other),
        }
    }
}
