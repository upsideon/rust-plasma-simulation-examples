use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use num_traits::identities::Zero;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x: x, y: y, z: z }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Zero for Vec3 {
    fn zero() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn is_zero(&self) -> bool {
        if self.x == 0.0 && self.y == 0.0 && self.z == 0.0 {
            return true;
        }
        return false;
    }

    fn set_zero(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T> Mul<T> for Vec3
where
    f64: From<T>,
    T: Copy,
{
    type Output = Vec3;

    fn mul(self, other: T) -> Self {
        Vec3 {
            x: self.x * f64::from(other),
            y: self.y * f64::from(other),
            z: self.z * f64::from(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let expected = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        assert_eq!(u, expected);
    }

    #[test]
    fn test_add() {
        let u = Vec3 {
            x: 3.0,
            y: -4.0,
            z: 5.5,
        };
        let v = Vec3 {
            x: 1.0,
            y: 8.0,
            z: -0.5,
        };
        let expected_sum = Vec3 {
            x: 4.0,
            y: 4.0,
            z: 5.0,
        };

        assert_eq!(u + v, expected_sum);
    }

    #[test]
    fn test_subtract() {
        let u = Vec3 {
            x: 3.0,
            y: -4.0,
            z: 5.5,
        };
        let v = Vec3 {
            x: 1.0,
            y: 8.0,
            z: -0.5,
        };
        let expected_difference = Vec3 {
            x: 2.0,
            y: -12.0,
            z: 6.0,
        };

        assert_eq!(u - v, expected_difference);
    }

    #[test]
    fn test_multiply() {
        let u = Vec3 {
            x: 3.0,
            y: -4.0,
            z: 5.5,
        };
        let v = Vec3 {
            x: 1.0,
            y: 8.0,
            z: -0.5,
        };
        let expected_product = Vec3 {
            x: 3.0,
            y: -32.0,
            z: -2.75,
        };

        assert_eq!(u * v, expected_product);
    }

    #[test]
    fn test_divide() {
        let u = Vec3 {
            x: 3.0,
            y: -4.0,
            z: 5.5,
        };
        let v = Vec3 {
            x: 1.0,
            y: 8.0,
            z: -0.5,
        };
        let expected_quotient = Vec3 {
            x: 3.0,
            y: -0.5,
            z: -11.0,
        };

        assert_eq!(u / v, expected_quotient);
    }

    #[test]
    fn test_scalar_multiply() {
        let u = Vec3 {
            x: 3.0,
            y: -4.0,
            z: 5.5,
        };
        let c = 3.0;
        let expected_result = Vec3 {
            x: 9.0,
            y: -12.0,
            z: 16.5,
        };

        assert_eq!(u * c, expected_result);
    }
}
