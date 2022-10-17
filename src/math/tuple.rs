const VECTOR: f64 = 0.0;
const POINT: f64 = 1.0;

use crate::math::util::epsilon_eq as feq;

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[allow(dead_code)]
impl Tuple{
    pub fn create(x: i64, y: i64, z: i64, w: i64) -> Self {
        Self {
            x: x as f64, y: y as f64, z: z as f64, w: w as f64
        }
    }
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {
            x, y, z, w
        }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self {
            x, y, z, w: VECTOR
        }
    }
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self {
            x, y, z, w: POINT
        }
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normal(&self) -> Self {
        let m = self.magnitude();
        Self {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
            w: self.w / m
        }
    }

    pub fn dot(&self, o: &Tuple) -> f64 {
        self.x * o.x
            + self.y * o.y
            + self.z * o.z
            + self.w * o.w
    }

    pub fn cross(&self, o: &Tuple) -> Self {
        Self {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.x * o.z,
            z: self.x * o.y - self.y * o.x,
            w: VECTOR
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, o: &Self) -> bool {
        feq(self.x, o.x) && feq(self.y, o.y) && feq(self.z, o.z) && feq(self.w, o.w)
    }
}

impl std::ops::Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
}

impl std::ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }
}

impl std::ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }
}

impl std::ops::Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, val: f64) -> Self::Output {
        Self {
            x: self.x * val,
            y: self.y * val,
            z: self.z * val,
            w: self.w * val
        }
    }
}

impl std::ops::Div<f64> for Tuple {
    type Output = Self;

    fn div(self, val: f64) -> Self::Output {
        Self {
            x: self.x / val,
            y: self.y / val,
            z: self.z / val,
            w: self.w / val
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tuple;

    #[test]
    fn scenario_a() {
        let t = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
        assert_eq!(t.w, 1.0);
        assert!(t.is_point());
        assert!(!t.is_vector());
    }

    #[test]
    fn scenario_b() {
        let t = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
        assert_eq!(t.w, 0.0);
        assert!(t.is_vector());
        assert!(!t.is_point());
    }

    #[test]
    fn point_constructor() {
        let p = Tuple::point(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple { x: 4.0, y: -4.0, z: 3.0, w: 1.0 });
        assert!(p.is_point());
    }

    #[test]
    fn vector_constructor() {
        let p = Tuple::vector(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple { x: 4.0, y: -4.0, z: 3.0, w: 0.0 });
        assert!(p.is_vector());
    }

    #[test]
    fn add() {
        let a = Tuple::create(3, -2, 5, 1);
        let b = Tuple::create(-2, 3, 1, 0);
        assert_eq!(a + b, Tuple::create(1, 1, 6, 1));
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
        let a = Tuple::create(1, -2, 3, -4);
        assert_eq!(-a, Tuple::create(-1, 2, -3, 4));
    }

    #[test]
    fn scalar_mul() {
        let a = Tuple::create(1, -2, 3, -4);
        assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
        assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn scalar_div() {
        let a = Tuple::create(1, -2, 3, -4);
        assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
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
        assert_eq!(v.normal(), Tuple::vector(1.0/(14.0f64).sqrt(), 2.0/(14.0f64).sqrt(), 3.0/(14.0f64).sqrt()));
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
}
