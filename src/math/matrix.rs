use crate::math::util::epsilon_eq as feq;
use std::ops::{Deref, DerefMut, Mul};

use super::{round, Tuple};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    rows: Vec<Vec<f64>>,
}

#[allow(dead_code)]
impl Matrix {
    pub fn new(rows: Vec<Vec<f64>>) -> Self {
        let height = rows.len();
        let width = if height > 0 { rows[0].len() } else { 0 };

        /* Sanity check to prevent borked matrixes */
        assert!(rows.iter().all(|r| r.len() == width));

        Self {
            width,
            height,
            rows,
        }
    }

    pub fn identity() -> Self {
        let height = 4;
        let width = 4;
        let mut rows = vec![vec![0.0; width]; height];
        rows[0][0] = 1.0;
        rows[1][1] = 1.0;
        rows[2][2] = 1.0;
        rows[3][3] = 1.0;

        Self {
            width,
            height,
            rows,
        }
    }

    pub fn with_dimension(w: usize, h: usize) -> Self {
        Self {
            width: w,
            height: h,
            rows: vec![vec![0.0; w]; h],
        }
    }

    pub fn col(&self, c: usize) -> Vec<f64> {
        self.rows.iter().map(|r| r[c]).collect()
    }

    pub fn transpose(&self) -> Self {
        let mut rows = vec![];
        for i in 0..self.width {
            rows.push(self.col(i));
        }
        Self {
            width: self.width,
            height: self.height,
            rows,
        }
    }

    pub fn determinant(&self) -> f64 {
        assert_eq!(self.width, self.height);

        if self.width == 2 {
            self.rows[0][0] * self.rows[1][1] - self.rows[0][1] * self.rows[1][0]
        } else {
            self.rows[0]
                .iter()
                .enumerate()
                .map(|(i, v)| v * self.cofactor(0, i))
                .sum()
        }
    }

    pub fn submatrix(&self, r: usize, c: usize) -> Self {
        let mut rows = self.rows.clone();
        rows.remove(r);
        rows.iter_mut().for_each(|row| {
            row.remove(c);
        });

        Self {
            width: self.width - 1,
            height: self.height - 1,
            rows,
        }
    }

    pub fn minor(&self, r: usize, c: usize) -> f64 {
        let sub = self.submatrix(r, c);
        sub.determinant()
    }

    pub fn cofactor(&self, r: usize, c: usize) -> f64 {
        let op = if (r + c) % 2 == 1 { -1.0 } else { 1.0 };
        self.minor(r, c) * op
    }

    pub fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Self {
        assert!(self.invertible());

        let mut n = Matrix::with_dimension(self.width, self.height);
        for i in 0..self.rows.len() {
            for j in 0..self.rows[i].len() {
                n[i][j] = self.cofactor(i, j);
            }
        }
        let determinant = self.determinant();
        let mut n = n.transpose();
        n.iter_mut().flatten().for_each(|v| {
            *v /= determinant;
        });
        n
    }

    pub fn round(&self, decimal_count: u32) -> Self {
        let mut clone = self.clone();
        clone
            .iter_mut()
            .flatten()
            .for_each(|v| *v = round(*v, decimal_count));
        clone
    }
}

impl Deref for Matrix {
    type Target = Vec<Vec<f64>>;

    fn deref(&self) -> &Self::Target {
        &self.rows
    }
}

impl DerefMut for Matrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rows
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        let mut zip_iter = self.rows.iter().flatten().zip(other.iter().flatten());
        zip_iter.all(|(a, b)| feq(*a, *b))
    }
}

macro_rules! matrix_mul {
    ($LHS:ty, $RHS:ty) => {
        impl Mul<$RHS> for $LHS {
            type Output = Matrix;

            fn mul(self, o: $RHS) -> Self::Output {
                let mut result = Matrix::with_dimension(self.width, self.height);
                for i in 0..self.height {
                    let m_row = self.rows[i].clone();
                    for j in 0..self.width {
                        let m_col = o.col(j);
                        let pairs = m_col.iter().zip(m_row.iter());
                        result.rows[i][j] = pairs.map(|(a, b)| a * b).sum();
                    }
                }
                result
            }
        }
    };
}

matrix_mul!(Matrix, Matrix);
matrix_mul!(Matrix, &Matrix);
matrix_mul!(&Matrix, &Matrix);
matrix_mul!(&Matrix, Matrix);

macro_rules! matrix_tuple_add {
    ($M:ty, $T:ty) => {
        impl Mul<$T> for $M {
            type Output = Tuple;

            fn mul(self, v: $T) -> Self::Output {
                let mut result: Vec<f64> = vec![0.0; self.height];
                for (i, row) in self.rows.iter().enumerate() {
                    result[i] = row.iter().zip(v.iter()).map(|(a, b)| a * b).sum::<f64>();
                }
                Tuple::from(result)
            }
        }
    };
}

matrix_tuple_add!(Matrix, Tuple);
matrix_tuple_add!(Matrix, &Tuple);
matrix_tuple_add!(&Matrix, &Tuple);
matrix_tuple_add!(&Matrix, Tuple);

#[cfg(test)]
mod test {
    use crate::math::Tuple;

    use super::Matrix;

    #[test]
    fn two_by_two() {
        let m = Matrix::new(vec![vec![-3.0, 5.0], vec![1.0, -2.0]]);
        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], -2.0);
    }

    #[test]
    fn four_by_four() {
        let m = Matrix::new(vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![9.0, 1.0, 1.0],
        ]);
        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    fn equality() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        assert_eq!(a, b);
    }

    #[test]
    fn inequality() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new(vec![
            vec![2.0, 3.0, 4.0, 5.0],
            vec![6.0, 7.0, 8.0, 9.0],
            vec![8.0, 7.0, 6.0, 5.0],
            vec![4.0, 3.0, 2.0, 1.0],
        ]);
        assert_ne!(a, b);
    }

    #[test]
    fn multiplication() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ]);
        let target = Matrix::new(vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ]);
        let result = a * b;
        assert_eq!(result, target);
    }

    #[test]
    fn vector_multiplication() {
        let m = Matrix::new(vec![
            vec![2.0, -1.0, 3.0, 5.0],
            vec![1.0, 3.0, 0.0, 4.0],
            vec![3.0, 0.0, -1.0, -2.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);
        let v = Tuple::from(&[2, 0, -1, 1]);
        let r = Tuple::from(&[6, 6, 5, 1]);
        assert_eq!(m * v, r);
    }

    #[test]
    fn vector_multiplication2() {
        let m = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);
        let v = Tuple::from(&[1, 2, 3, 1]);
        let r = Tuple::from(&[18, 24, 33, 1]);
        assert_eq!(m * v, r);
    }

    #[test]
    fn identity_matrix() {
        let m = Matrix::new(vec![
            vec![0.0, 1.0, 2.0, 3.0],
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 8.0, 16.0, 32.0],
        ]);
        assert_eq!(&m * &Matrix::identity(), m);
    }

    #[test]
    fn transpose() {
        let m = Matrix::new(vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ]);
        let t = Matrix::new(vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        ]);
        assert_eq!(m.transpose(), t);
    }

    #[test]
    fn transpose_identity() {
        assert_eq!(Matrix::identity().transpose(), Matrix::identity());
    }

    #[test]
    fn determinant() {
        let m = Matrix::new(vec![vec![1.0, 5.0], vec![-3.0, 2.0]]);
        assert_eq!(m.determinant(), 17.0);
    }

    #[test]
    fn submatrix_3x3() {
        let m = Matrix::new(vec![
            vec![1.0, 5.0, 0.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0],
        ]);
        let t = Matrix::new(vec![vec![-3.0, 2.0], vec![0.0, 6.0]]);

        assert_eq!(m.submatrix(0, 2), t);
    }

    #[test]
    fn submatrix_4x4() {
        let m = Matrix::new(vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0],
        ]);
        let t = Matrix::new(vec![
            vec![-6.0, 1.0, 6.0],
            vec![-8.0, 8.0, 6.0],
            vec![-7.0, -1.0, 1.0],
        ]);

        assert_eq!(m.submatrix(2, 1), t);
    }

    #[test]
    fn determinant_3x3() {
        let m = Matrix::new(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);
        let subm = m.submatrix(1, 0);
        assert_eq!(subm.determinant(), 25.0);
        assert_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor_3x3() {
        let m = Matrix::new(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);
        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn determinant_3x3_2() {
        let m = Matrix::new(vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ]);
        assert_eq!(m.cofactor(0, 0), 56.0);
        assert_eq!(m.cofactor(0, 1), 12.0);
        assert_eq!(m.cofactor(0, 2), -46.0);
        assert_eq!(m.determinant(), -196.0);
    }

    #[test]
    fn determinant_4x4() {
        let m = Matrix::new(vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.cofactor(0, 1), 447.0);
        assert_eq!(m.cofactor(0, 2), 210.0);
        assert_eq!(m.cofactor(0, 3), 51.0);
        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn invertible() {
        let m = Matrix::new(vec![
            vec![6.0, 4.0, 4.0, 4.0],
            vec![5.0, 5.0, 7.0, 6.0],
            vec![4.0, -9.0, 3.0, -7.0],
            vec![9.0, 1.0, 7.0, -6.0],
        ]);
        assert_eq!(m.determinant(), -2120.0);
        assert!(m.invertible());
    }

    #[test]
    fn not_invertible() {
        let m = Matrix::new(vec![
            vec![-4.0, 2.0, -2.0, -3.0],
            vec![9.0, 6.0, 2.0, 6.0],
            vec![0.0, -5.0, 1.0, -5.0],
            vec![0.0, 0.0, 0.0, -0.0],
        ]);
        assert_eq!(m.determinant(), 0.0);
        assert!(!m.invertible());
    }

    #[test]
    fn inverse() {
        let a = Matrix::new(vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ]);
        let t = Matrix::new(vec![
            vec![0.21805, 0.45113, 0.24060, -0.04511],
            vec![-0.80827, -1.45677, -0.44361, 0.52068],
            vec![-0.07895, -0.22368, -0.05263, 0.19737],
            vec![-0.52256, -0.81391, -0.30075, 0.30639],
        ]);
        let b = a.inverse();
        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert_eq!(b[3][2], -160.0 / 532.0);
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert_eq!(b[2][3], 105.0 / 532.0);
        assert_eq!(b.round(5), t);
    }
}
