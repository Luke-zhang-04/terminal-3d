use std::f64::consts::PI;
use std::fmt;
use std::ops;

use crate::vector3::Vector3;

/// 3x3 square matrix
/// We won't be needing any other matrix size (except perhaps 2x2), so we don't really need to generalize
#[derive(Clone, Copy)]
pub struct Matrix3 {
    pub mat: [[f64; 3]; 3],
}

#[macro_export]
macro_rules! matrix3 {
    ($a:expr, $b:expr, $c:expr) => {
        Matrix3 {
            mat: [
                [$a.0 as f64, $a.1 as f64, $a.2 as f64],
                [$b.0 as f64, $b.1 as f64, $b.2 as f64],
                [$c.0 as f64, $c.1 as f64, $c.2 as f64],
            ],
        }
    };
}

/// Macro to repeat something 3x3 times to simplify writing while also allowing compile-time unrolling
macro_rules! repeat_3x3 {
    ($body:expr) => {
        [
            [$body(0, 0), $body(0, 1), $body(0, 2)],
            [$body(1, 0), $body(1, 1), $body(1, 2)],
            [$body(2, 0), $body(2, 1), $body(2, 2)],
        ]
    };
}

macro_rules! repeat_row {
    ($body:expr) => {
        [$body(0), $body(1), $body(2)]
    };
}

impl Matrix3 {
    pub fn new(mat: [[f64; 3]; 3]) -> Matrix3 {
        Matrix3 { mat }
    }

    pub fn from_i64(mat: [[i64; 3]; 3]) -> Matrix3 {
        Matrix3 {
            mat: repeat_3x3!(|row: usize, col: usize| mat[row][col] as f64),
        }
    }

    /// Uses 3 vectors as column vectors
    pub fn from_vec3(mat: (Vector3, Vector3, Vector3)) -> Matrix3 {
        Matrix3 {
            mat: [
                [mat.0.x, mat.1.x, mat.2.x],
                [mat.0.y, mat.1.y, mat.2.y],
                [mat.0.z, mat.1.z, mat.2.z],
            ],
        }
    }

    /// Zero matrix
    pub fn zero() -> Matrix3 {
        matrix3!((0, 0, 0), (0, 0, 0), (0, 0, 0))
    }

    /// Identity matrix
    pub fn id() -> Matrix3 {
        matrix3!((1, 0, 0), (0, 1, 0), (0, 0, 1))
    }

    pub fn row(&self, row: usize) -> Vector3 {
        Vector3::from(self.mat[row])
    }

    pub fn set_row(&mut self, row: usize, vec: Vector3) {
        self.mat[row][0] = vec.x;
        self.mat[row][1] = vec.y;
        self.mat[row][2] = vec.z;
    }

    pub fn col(&self, col: usize) -> Vector3 {
        Vector3::new(self.mat[0][col], self.mat[1][col], self.mat[2][col])
    }

    pub fn set_col(&mut self, col: usize, vec: Vector3) {
        self.mat[0][col] = vec.x;
        self.mat[1][col] = vec.y;
        self.mat[2][col] = vec.z;
    }

    pub fn transpose(&self) -> Matrix3 {
        // I could've used a for loop I guess, but this probably performs better or something
        Matrix3 {
            mat: repeat_3x3!(|row: usize, col: usize| self.mat[col][row]),
        }
    }

    pub fn is_symmetric(self) -> bool {
        self == self.transpose()
    }

    pub fn determinant(&self) -> f64 {
        self.mat[0][0] * self.mat[1][1] * self.mat[2][2]
            + self.mat[0][1] * self.mat[1][2] * self.mat[2][0]
            + self.mat[0][2] * self.mat[1][0] * self.mat[2][1]
            - self.mat[0][2] * self.mat[1][1] * self.mat[2][0]
            - self.mat[0][0] * self.mat[1][2] * self.mat[2][1]
            - self.mat[0][1] * self.mat[1][0] * self.mat[2][2]
    }

    pub fn is_singular(self) -> bool {
        self.determinant().abs() < f64::EPSILON
    }

    pub fn is_orthonormal(&self) -> bool {
        for row in 0..=2 {
            let vec = Vector3::from(self.mat[row]);
            if (vec.magnitude() - 1.0).abs() >= f64::EPSILON {
                return false;
            }

            for compare in row + 1..=2 {
                if (vec.angle(Vector3::from(self.mat[compare])).abs() - (PI / 2.0)).abs()
                    >= f64::EPSILON
                {
                    return false;
                }
            }
        }

        true
    }

    pub fn normalize_rows(self) -> Matrix3 {
        Matrix3 {
            mat: repeat_row!(|row: usize| self.row(row).normalize().components()),
        }
    }

    pub fn normalize_cols(self) -> Matrix3 {
        let mut mat = Matrix3::zero();

        for col in 0..=2 {
            let vec = self.col(col).normalize();

            mat[0][col] = vec.x;
            mat[1][col] = vec.y;
            mat[2][col] = vec.z;
        }

        mat
    }

    /// Orthonormalize using Gram-Schmidt, normalizing each column as we go
    /// Assumes columns of matrix are linearly independent
    pub fn orthonormalize(&self) -> Matrix3 {
        let mut mat = self.clone();

        mat.set_col(0, mat.col(0).normalize());

        for col in 1..=2 {
            let original = mat.col(col);
            let mut vec = mat.col(col);

            for prev in 0..col {
                vec -= original.project(mat.col(prev));
            }

            mat.set_col(col, vec.normalize());
        }

        mat
    }

    /// Get the minor of the i-th row and j-th column
    /// Instead of deleting the i-th row and j-th column, we can just set 1s and 0s to make it look like a 2x2 matrix
    pub fn minor(&self, i: usize, j: usize) -> f64 {
        let mut mat = self.clone();

        for row in 0..=2 {
            mat.mat[row][j] = 0.0
        }
        for col in 0..=2 {
            if col == j {
                mat.mat[i][col] = if (i + col) % 2 == 0 { 1.0 } else { -1.0 }
            } else {
                mat.mat[i][col] = 0.0
            }
        }

        mat.determinant()
    }

    /// Get the cofactor of the i-th row and j-th column
    pub fn cofactor(&self, i: usize, j: usize) -> f64 {
        let is_even = (i + j) % 2 == 0;

        if is_even {
            self.minor(i, j)
        } else {
            -self.minor(i, j)
        }
    }

    pub fn cofactor_matrix(&self) -> Matrix3 {
        Matrix3 {
            mat: repeat_3x3!(|row: usize, col: usize| self.cofactor(row, col)),
        }
    }

    pub fn adjugate(&self) -> Matrix3 {
        self.cofactor_matrix().transpose()
    }

    pub fn invert(&self) -> Option<Matrix3> {
        let det = self.determinant();

        if det.abs() < f64::EPSILON {
            None
        } else {
            Some((1.0 / det) * self.adjugate())
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "[ {:?}, {:?}, {:?} ]",
            self.mat[0], self.mat[1], self.mat[2]
        )
    }
}

impl ops::Add<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn add(self, rhs: Matrix3) -> Self::Output {
        Matrix3 {
            mat: repeat_3x3!(|row: usize, col: usize| self.mat[row][col] + rhs.mat[row][col]),
        }
    }
}

impl ops::AddAssign<Matrix3> for Matrix3 {
    fn add_assign(&mut self, rhs: Matrix3) {
        *self = *self + rhs;
    }
}

impl ops::Div<f64> for Matrix3 {
    type Output = Matrix3;

    fn div(self, scalar: f64) -> Self::Output {
        self * (1.0 / scalar)
    }
}

impl ops::DivAssign<f64> for Matrix3 {
    fn div_assign(&mut self, scalar: f64) {
        *self = *self / scalar
    }
}

// Scalar multiplication
impl ops::Mul<f64> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, scalar: f64) -> Self::Output {
        Matrix3 {
            mat: repeat_3x3!(|row: usize, col: usize| self.mat[row][col] * scalar),
        }
    }
}

impl ops::MulAssign<f64> for Matrix3 {
    fn mul_assign(&mut self, scalar: f64) {
        *self = *self * scalar
    }
}

impl ops::Mul<Matrix3> for f64 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        rhs.mul(self)
    }
}

// Vector product
impl ops::Mul<Vector3> for Matrix3 {
    type Output = Vector3;

    fn mul(self, vec: Vector3) -> Self::Output {
        Vector3::from(repeat_row!(|row: usize| self.mat[row][0] * vec.x
            + self.mat[row][1] * vec.y
            + self.mat[row][2] * vec.z))
    }
}

// Matrix product
impl ops::Mul<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        Matrix3 {
            mat: repeat_3x3!(|row: usize, col: usize| self.mat[row][0] * rhs.mat[0][col]
                + self.mat[row][1] * rhs.mat[1][col]
                + self.mat[row][2] * rhs.mat[2][col]),
        }
    }
}

impl ops::Sub<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn sub(self, rhs: Matrix3) -> Self::Output {
        self + -rhs
    }
}

impl ops::SubAssign<Matrix3> for Matrix3 {
    fn sub_assign(&mut self, rhs: Matrix3) {
        *self = *self - rhs
    }
}

impl ops::Neg for Matrix3 {
    type Output = Matrix3;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl ops::Index<usize> for Matrix3 {
    type Output = [f64; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.mat[index]
    }
}

impl ops::IndexMut<usize> for Matrix3 {
    fn index_mut(&mut self, index: usize) -> &mut [f64; 3] {
        &mut self.mat[index]
    }
}

impl PartialEq for Matrix3 {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..=2 {
            for col in 0..=2 {
                if (self.mat[row][col] - other.mat[row][col]).abs() >= f64::EPSILON {
                    return false;
                }
            }
        }

        true
    }
}

impl fmt::Debug for Matrix3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Pretty print a matrix
impl fmt::Display for Matrix3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut strings: [[String; 3]; 3] = Default::default();
        let mut max_len: [usize; 3] = Default::default(); // Max length per column

        for row in 0..=2 {
            for col in 0..=2 {
                strings[row][col] = format!("{}", self.mat[row][col]);

                if strings[row][col].len() > max_len[col] {
                    max_len[col] = strings[row][col].len()
                }
            }
        }

        let mut rows: [String; 3] = Default::default();

        for row in 0..=2 {
            rows[row] = format!(
                "{}{} {}{} {}{}",
                strings[row][0],
                " ".repeat(max_len[0] - strings[row][0].len()),
                strings[row][1],
                " ".repeat(max_len[1] - strings[row][1].len()),
                strings[row][2],
                " ".repeat(max_len[2] - strings[row][2].len()),
            )
        }

        let spaces = max_len[0] + max_len[1] + max_len[2];

        write!(
            f,
            "--{}--\n| {} |\n| {} |\n| {} |\n--{}--",
            " ".repeat(spaces + 2),
            rows[0],
            rows[1],
            rows[2],
            " ".repeat(spaces + 2),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn rotation_matrix() {
        let angle = PI / 4.0; // 45 degrees
        let mat = matrix3!(
            (angle.cos(), angle.sin(), 0),
            (-angle.sin(), angle.cos(), 0),
            (0, 0, 1)
        );
        assert_eq!(mat.determinant(), 1.0);
        assert_eq!(mat.transpose(), mat.invert().unwrap());
        assert_eq!(mat * mat.invert().unwrap(), Matrix3::id());
        // Due to floating-point precision issues, this assert won't work
        // assert_eq!(mat * mat, matrix3!((0, 1, 0), (-1, 0, 0), (0, 0, 1)));
        assert_eq!(mat.is_orthonormal(), true);
        assert_eq!(mat.orthonormalize(), mat);
    }

    #[test]
    fn orthonormalize() {
        let mat = matrix3!(
            (1, 67, 10), // I'm cooked
            (0, 67, f64::EPSILON),
            (0, 0, i32::MAX)
        );

        assert_eq!(mat.is_orthonormal(), false);
        assert_eq!(mat.orthonormalize().is_orthonormal(), true);
        // Happens to be the case with this particular matrix using the Gram-Schmidt procedure like this
        assert_eq!(mat.orthonormalize(), Matrix3::id());
    }
}
