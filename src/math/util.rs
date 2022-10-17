pub fn epsilon_eq(a: f64 , b: f64) -> bool {
    if (a-b).abs() < std::f64::EPSILON {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use super::epsilon_eq;

    #[test]
    fn float_compare() {
        assert_ne!(0.1f64 + 0.2f64, 0.3f64);
    }

    #[test]
    fn espilon_compare() {
        assert!(epsilon_eq(0.1f64+0.2f64, 0.3f64));
    }
}
