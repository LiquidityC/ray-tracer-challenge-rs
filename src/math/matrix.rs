use crate::math::util::epsilon_eq as feq;
use std::ops::{Deref, DerefMut, Mul};

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

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, o: Self) -> Self::Output {
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

#[cfg(test)]
mod test {
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
}
