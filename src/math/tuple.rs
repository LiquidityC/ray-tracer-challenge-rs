const VECTOR: f64 = 0.0;
const POINT: f64 = 1.0;

use std::ops::{Deref, DerefMut};

use crate::math::util::epsilon_eq as feq;

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    vector: [f64; 4],
}

#[allow(dead_code)]
impl Tuple {
    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self {
            vector: [x, y, z, VECTOR],
        }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self {
            vector: [x, y, z, POINT],
        }
    }

    pub fn color(r: f64, g: f64, b: f64) -> Self {
        Self {
            vector: [r, g, b, 0.0],
        }
    }

    pub fn x(&self) -> f64 {
        self.vector[0]
    }

    pub fn y(&self) -> f64 {
        self.vector[1]
    }

    pub fn z(&self) -> f64 {
        self.vector[2]
    }

    pub fn w(&self) -> f64 {
        self.vector[3]
    }

    pub fn red(&self) -> f64 {
        self.vector[0]
    }

    pub fn green(&self) -> f64 {
        self.vector[1]
    }

    pub fn blue(&self) -> f64 {
        self.vector[2]
    }

    pub fn is_vector(&self) -> bool {
        self.vector[3] == 0.0
    }

    pub fn is_point(&self) -> bool {
        self.vector[3] == 1.0
    }

    pub fn magnitude(&self) -> f64 {
        self.vector.iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
    }

    pub fn normal(&self) -> Self {
        let m = self.magnitude();
        Self {
            vector: self
                .vector
                .iter()
                .map(|x| x / m)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn dot(&self, o: &Tuple) -> f64 {
        self.vector.iter().zip(o.iter()).map(|(a, b)| a * b).sum()
    }

    pub fn cross(&self, o: &Tuple) -> Self {
        assert_eq!(self.vector.len(), 4);
        assert_eq!(o.vector.len(), 4);
        Self {
            vector: [
                self.y() * o.z() - self.z() * o.y(),
                self.z() * o.x() - self.x() * o.z(),
                self.x() * o.y() - self.y() * o.x(),
                VECTOR,
            ],
        }
    }
}

impl Default for Tuple {
    fn default() -> Self {
        Self { vector: [0.0; 4] }
    }
}

impl Deref for Tuple {
    type Target = [f64; 4];

    fn deref(&self) -> &Self::Target {
        &self.vector
    }
}

impl DerefMut for Tuple {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vector
    }
}

impl From<Vec<f64>> for Tuple {
    fn from(vector: Vec<f64>) -> Self {
        Self {
            vector: vector.try_into().unwrap(),
        }
    }
}

impl From<&[f64; 4]> for Tuple {
    fn from(vector: &[f64; 4]) -> Self {
        Self {
            vector: vector.to_owned(),
        }
    }
}

impl From<&[i64; 4]> for Tuple {
    fn from(vector: &[i64; 4]) -> Self {
        Self {
            vector: vector
                .iter()
                .map(|x| *x as f64)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, o: &Self) -> bool {
        self.vector
            .iter()
            .zip(o.vector.iter())
            .all(|(a, b)| feq(*a, *b))
    }
}

impl std::ops::Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            vector: self
                .vector
                .iter()
                .zip(rhs.vector.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl std::ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            vector: self
                .vector
                .iter()
                .zip(rhs.vector.iter())
                .map(|(a, b)| a - b)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl std::ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            vector: self
                .vector
                .iter()
                .map(|x| -x)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl std::ops::Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, val: f64) -> Self::Output {
        Self {
            vector: self
                .vector
                .iter()
                .map(|x| x * val)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl std::ops::Mul<Tuple> for Tuple {
    type Output = Self;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Self {
            vector: self
                .vector
                .iter()
                .zip(rhs.vector.iter())
                .map(|(a, b)| a * b)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl std::ops::Div<f64> for Tuple {
    type Output = Self;

    fn div(self, val: f64) -> Self::Output {
        Self {
            vector: self
                .vector
                .iter()
                .map(|x| x / val)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tuple;

    #[test]
    fn scenario_a() {
        let t = Tuple::from(&[4.3, -4.2, 3.1, 1.0]);
        assert_eq!(t.x(), 4.3);
        assert_eq!(t.y(), -4.2);
        assert_eq!(t.z(), 3.1);
        assert_eq!(t.w(), 1.0);
        assert!(t.is_point());
        assert!(!t.is_vector());
    }

    #[test]
    fn scenario_b() {
        let t = Tuple::from(&[4.3, -4.2, 3.1, 0.0]);
        assert_eq!(t.x(), 4.3);
        assert_eq!(t.y(), -4.2);
        assert_eq!(t.z(), 3.1);
        assert_eq!(t.w(), 0.0);
        assert!(t.is_vector());
        assert!(!t.is_point());
    }

    #[test]
    fn point_constructor() {
        let p = Tuple::point(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple::from(&[4.0, -4.0, 3.0, 1.0]));
        assert!(p.is_point());
    }

    #[test]
    fn vector_constructor() {
        let p = Tuple::vector(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple::from(&[4, -4, 3, 0]));
        assert!(p.is_vector());
    }

    #[test]
    fn add() {
        let a = Tuple::from(&[3, -2, 5, 1]);
        let b = Tuple::from(&[-2, 3, 1, 0]);
        assert_eq!(a + b, Tuple::from(&[1, 1, 6, 1]));
    }

    #[test]
    fn sub_point_from_point() {
        let a = Tuple::point(3.0, 2.0, 1.0);
        let b = Tuple::point(5.0, 6.0, 7.0);
        assert_eq!(a - b, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn sub_vector_from_point() {
        let a = Tuple::point(3.0, 2.0, 1.0);
        let b = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(a - b, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_sub_vector_from_vector() {
        let a = Tuple::vector(3.0, 2.0, 1.0);
        let b = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(a - b, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn sub_vector_from_zero_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);
        assert_eq!(zero - v, Tuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negate_tuple() {
        let a = Tuple::from(&[1, -2, 3, -4]);
        assert_eq!(-a, Tuple::from(&[-1, 2, -3, 4]));
    }

    #[test]
    fn scalar_mul() {
        let a = Tuple::from(&[1, -2, 3, -4]);
        assert_eq!(a * 3.5, Tuple::from(&[3.5, -7.0, 10.5, -14.0]));
        assert_eq!(a * 0.5, Tuple::from(&[0.5, -1.0, 1.5, -2.0]));
    }

    #[test]
    fn scalar_div() {
        let a = Tuple::from(&[1, -2, 3, -4]);
        assert_eq!(a / 2.0, Tuple::from(&[0.5, -1.0, 1.5, -2.0]));
    }

    #[test]
    fn magnitude() {
        let v = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = Tuple::vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), (14f64).sqrt());

        let v = Tuple::vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), (14f64).sqrt());
    }

    #[test]
    fn normalization() {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        assert_eq!(v.normal(), Tuple::vector(1.0, 0.0, 0.0));

        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(
            v.normal(),
            Tuple::vector(
                1.0 / (14.0f64).sqrt(),
                2.0 / (14.0f64).sqrt(),
                3.0 / (14.0f64).sqrt()
            )
        );
    }

    #[test]
    fn normal_magnitude() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        let norm = v.normal();
        assert_eq!(norm.magnitude(), 1.0);
    }

    #[test]
    fn dot_product() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn cross_product() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(a.cross(&b), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(&a), Tuple::vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn color() {
        let c = Tuple::color(-0.5, 0.4, 1.7);
        assert_eq!(c.red(), -0.5);
        assert_eq!(c.green(), 0.4);
        assert_eq!(c.blue(), 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = Tuple::color(0.9, 0.6, 0.75);
        let c2 = Tuple::color(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Tuple::color(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Tuple::color(0.9, 0.6, 0.75);
        let c2 = Tuple::color(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, Tuple::color(0.2, 0.5, 0.5));
    }

    #[test]
    fn scalar_color_multiplication() {
        let c1 = Tuple::color(1.0, 0.3, 0.4);
        assert_eq!(c1 * 2.0, Tuple::color(2.0, 0.6, 0.8));
    }

    #[test]
    fn color_multiplication() {
        let c1 = Tuple::color(1.0, 0.2, 0.4);
        let c2 = Tuple::color(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, Tuple::color(0.9, 0.2, 0.04));
    }
}
