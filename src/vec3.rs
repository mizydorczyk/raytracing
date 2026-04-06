use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.square().sqrt()
    }

    pub fn square(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(_u: Self, _v: Self) -> f64 {
        _u.x * _v.x + _u.y * _v.y + _u.z * _v.z
    }

    pub fn cross(_u: Self, _v: Self) -> Self {
        Self {
            x: _u.y * _v.z - _u.z * _v.y,
            y: _u.z * _v.x - _u.x * _v.z,
            z: _u.x * _v.y - _u.y * _v.x,
        }
    }

    pub fn unit(_v: Self) -> Self {
        _v / _v.length()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul for Vec3 {
    type Output = Self; // component-wise

    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, factor: f64) -> Self::Output {
        Self::new(self.x * factor, self.y * factor, self.z * factor)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Self::Output {
        vector * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, factor: f64) -> Self::Output {
        self * (1.0 / factor)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, factor: f64) {
        *self *= 1.0 / factor;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-12;

    fn assert_f64_eq(a: f64, b: f64) {
        assert!(
            (a - b).abs() < EPSILON,
            "expected {a} to be approximately equal to {b}"
        );
    }

    #[test]
    fn negates_vector() {
        let v = Vec3::new(1.0, -2.0, 3.5);
        let n = -v;

        assert_f64_eq(n.x, -1.0);
        assert_f64_eq(n.y, 2.0);
        assert_f64_eq(n.z, -3.5);
    }

    #[test]
    fn adds_vectors_component_wise() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let c = a + b;

        assert_f64_eq(c.x, 5.0);
        assert_f64_eq(c.y, 7.0);
        assert_f64_eq(c.z, 9.0);
    }

    #[test]
    fn subtracts_vectors_component_wise() {
        let a = Vec3::new(7.0, 8.0, 9.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        let c = a - b;

        assert_f64_eq(c.x, 6.0);
        assert_f64_eq(c.y, 6.0);
        assert_f64_eq(c.z, 6.0);
    }

    #[test]
    fn multiplies_vectors_component_wise() {
        let a = Vec3::new(2.0, 3.0, 4.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        let c = a * b;

        assert_f64_eq(c.x, 10.0);
        assert_f64_eq(c.y, 18.0);
        assert_f64_eq(c.z, 28.0);
    }

    #[test]
    fn multiplies_vector_by_scalar_on_right() {
        let v = Vec3::new(1.5, -2.0, 4.0);
        let r = v * 2.0;

        assert_f64_eq(r.x, 3.0);
        assert_f64_eq(r.y, -4.0);
        assert_f64_eq(r.z, 8.0);
    }

    #[test]
    fn multiplies_vector_by_scalar_on_left() {
        let v = Vec3::new(1.5, -2.0, 4.0);
        let r = 2.0 * v;

        assert_f64_eq(r.x, 3.0);
        assert_f64_eq(r.y, -4.0);
        assert_f64_eq(r.z, 8.0);
    }

    #[test]
    fn divides_vector_by_scalar() {
        let v = Vec3::new(3.0, -6.0, 9.0);
        let r = v / 3.0;

        assert_f64_eq(r.x, 1.0);
        assert_f64_eq(r.y, -2.0);
        assert_f64_eq(r.z, 3.0);
    }

    #[test]
    fn add_assign_updates_vector() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v += Vec3::new(4.0, 5.0, 6.0);

        assert_f64_eq(v.x, 5.0);
        assert_f64_eq(v.y, 7.0);
        assert_f64_eq(v.z, 9.0);
    }

    #[test]
    fn mul_assign_updates_vector() {
        let mut v = Vec3::new(1.5, -2.0, 4.0);
        v *= 2.0;

        assert_f64_eq(v.x, 3.0);
        assert_f64_eq(v.y, -4.0);
        assert_f64_eq(v.z, 8.0);
    }

    #[test]
    fn div_assign_updates_vector() {
        let mut v = Vec3::new(3.0, -6.0, 9.0);
        v /= 3.0;

        assert_f64_eq(v.x, 1.0);
        assert_f64_eq(v.y, -2.0);
        assert_f64_eq(v.z, 3.0);
    }

    #[test]
    fn new_sets_components() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_f64_eq(v.x, 1.0);
        assert_f64_eq(v.y, 2.0);
        assert_f64_eq(v.z, 3.0);
    }

    #[test]
    fn length_is_sqrt_of_length_squared() {
        let v = Vec3::new(2.0, 3.0, 6.0);

        assert_f64_eq(v.length(), 7.0);
    }

    #[test]
    fn length_squared_computes_sum_of_squares() {
        let v = Vec3::new(2.0, 3.0, 6.0);
        assert_f64_eq(v.square(), 49.0);
    }

    #[test]
    fn dot_product_matches_definition() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(4.0, 5.0, 6.0);

        assert_f64_eq(Vec3::dot(u, v), 32.0);
    }

    #[test]
    fn cross_product_matches_definition() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(4.0, 5.0, 6.0);
        let c = Vec3::cross(u, v);

        assert_f64_eq(c.x, -3.0);
        assert_f64_eq(c.y, 6.0);
        assert_f64_eq(c.z, -3.0);
    }

    #[test]
    fn unit_vector_has_length_one() {
        let v = Vec3::new(0.0, 3.0, 4.0);
        let u = Vec3::unit(v);

        assert_f64_eq(u.length(), 1.0);
    }
}

pub type Point = Vec3;
pub type Color = Vec3;
