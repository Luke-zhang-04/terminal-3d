use std::fmt;
use std::ops;

#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[macro_export]
macro_rules! vector3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vector3 {
            x: $x as f64,
            y: $y as f64,
            z: $z as f64,
        }
    };
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn from_i64(x: i64, y: i64, z: i64) -> Vector3 {
        Vector3 {
            x: x as f64,
            y: y as f64,
            z: z as f64,
        }
    }

    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// P-norm. Magnitude is the Euclidean 2-norm
    pub fn norm(&self, p: i32) -> f64 {
        (self.x.powi(p) + self.y.powi(p) + self.z.powi(p)).powf(1.0 / (p as f64))
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn components(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    pub fn normalize(&self) -> Vector3 {
        let mag = self.magnitude();

        Vector3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn dot(&self, rhs: &Vector3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn angle(&self, other: &Vector3) -> f64 {
        (self.dot(other) / (self.magnitude() * other.magnitude())).acos()
    }

    /// Check if magnitude is less than the machine epsilon for 64-bit floating point
    pub fn is_zero(self) -> bool {
        self.magnitude() < f64::EPSILON
    }

    pub fn distance_to(self, other: Vector3) -> f64 {
        (self - other).magnitude()
    }

    pub fn project(self, onto: Vector3) -> Vector3 {
        // Project u onto v = ((u dot v) / |v|^2) * v
        // |v|^2 = v dot v
        onto * (self.dot(&onto) / onto.dot(&onto))
    }

    pub fn to_string(&self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        *self = Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, scalar: f64) -> Self::Output {
        Vector3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, scalar: f64) {
        *self = Vector3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f64) -> Self::Output {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, scalar: f64) {
        *self = Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        rhs.mul(self)
    }
}

/// Cross product
impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.y * rhs.z - self.z - rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y - rhs.x,
        }
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        self + -rhs
    }
}

impl ops::SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        *self = Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f64::EPSILON
            && (self.y - other.y).abs() < f64::EPSILON
            && (self.z - other.z).abs() < f64::EPSILON
    }
}

impl From<(f64, f64, f64)> for Vector3 {
    fn from(tuple: (f64, f64, f64)) -> Self {
        Vector3 {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

impl From<(i64, i64, i64)> for Vector3 {
    fn from(tuple: (i64, i64, i64)) -> Self {
        Vector3 {
            x: tuple.0 as f64,
            y: tuple.1 as f64,
            z: tuple.2 as f64,
        }
    }
}

impl From<[f64; 3]> for Vector3 {
    fn from(arr: [f64; 3]) -> Self {
        Vector3 {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }
    }
}

impl From<[i64; 3]> for Vector3 {
    fn from(arr: [i64; 3]) -> Self {
        Vector3 {
            x: arr[0] as f64,
            y: arr[1] as f64,
            z: arr[2] as f64,
        }
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
