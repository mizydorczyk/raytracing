use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    components: [f64; 3],
}

impl Vec3 {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self {
            components: [a, b, c],
        }
    }

    pub fn length(&self) -> f64 {
        self.square().sqrt()
    }

    pub fn square(&self) -> f64 {
        self.components[0] * self.components[0]
            + self.components[1] * self.components[1]
            + self.components[2] * self.components[2]
    }

    pub fn dot(u: Self, v: Self) -> f64 {
        u.components[0] * v.components[0]
            + u.components[1] * v.components[1]
            + u.components[2] * v.components[2]
    }

    pub fn cross(u: Self, v: Self) -> Self {
        Self {
            components: [
                u.components[1] * v.components[2] - u.components[2] * v.components[1],
                u.components[2] * v.components[0] - u.components[0] * v.components[2],
                u.components[0] * v.components[1] - u.components[1] * v.components[0],
            ],
        }
    }

    pub fn unit(v: Self) -> Self {
        v / v.length()
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.components[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.components[index]
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            components: [
                self.components[0] + other.components[0],
                self.components[1] + other.components[1],
                self.components[2] + other.components[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            components: [
                self.components[0] - other.components[0],
                self.components[1] - other.components[1],
                self.components[2] - other.components[2],
            ],
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            components: [
                self.components[0] * other.components[0],
                self.components[1] * other.components[1],
                self.components[2] * other.components[2],
            ],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, factor: f64) -> Self::Output {
        Self {
            components: [
                self.components[0] * factor,
                self.components[1] * factor,
                self.components[2] * factor,
            ],
        }
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
        Self {
            components: [
                -self.components[0],
                -self.components[1],
                -self.components[2],
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.components[0] += other.components[0];
        self.components[1] += other.components[1];
        self.components[2] += other.components[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, factor: f64) {
        self.components[0] *= factor;
        self.components[1] *= factor;
        self.components[2] *= factor;
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
    fn array_indexing() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_f64_eq(v[0], 1.0);
        assert_f64_eq(v[1], 2.0);
        assert_f64_eq(v[2], 3.0);
    }

    #[test]
    fn mutable_array_indexing() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v[0] = 5.0;
        v[1] = 6.0;
        v[2] = 7.0;
        assert_f64_eq(v[0], 5.0);
        assert_f64_eq(v[1], 6.0);
        assert_f64_eq(v[2], 7.0);
    }

    #[test]
    fn negates_vector() {
        let v = Vec3::new(1.0, -2.0, 3.5);
        let n = -v;

        assert_f64_eq(n[0], -1.0);
        assert_f64_eq(n[1], 2.0);
        assert_f64_eq(n[2], -3.5);
    }

    #[test]
    fn adds_vectors_component_wise() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(4.0, 5.0, 6.0);
        let w = u + v;

        assert_f64_eq(w[0], 5.0);
        assert_f64_eq(w[1], 7.0);
        assert_f64_eq(w[2], 9.0);
    }

    #[test]
    fn subtracts_vectors_component_wise() {
        let u = Vec3::new(7.0, 8.0, 9.0);
        let v = Vec3::new(1.0, 2.0, 3.0);
        let w = u - v;

        assert_f64_eq(w[0], 6.0);
        assert_f64_eq(w[1], 6.0);
        assert_f64_eq(w[2], 6.0);
    }

    #[test]
    fn multiplies_vectors_component_wise() {
        let u = Vec3::new(2.0, 3.0, 4.0);
        let v = Vec3::new(5.0, 6.0, 7.0);
        let w = u * v;

        assert_f64_eq(w[0], 10.0);
        assert_f64_eq(w[1], 18.0);
        assert_f64_eq(w[2], 28.0);
    }

    #[test]
    fn multiplies_vector_by_scalar_on_right() {
        let v = Vec3::new(1.5, -2.0, 4.0);
        let r = v * 2.0;

        assert_f64_eq(r[0], 3.0);
        assert_f64_eq(r[1], -4.0);
        assert_f64_eq(r[2], 8.0);
    }

    #[test]
    fn multiplies_vector_by_scalar_on_left() {
        let v = Vec3::new(1.5, -2.0, 4.0);
        let r = 2.0 * v;

        assert_f64_eq(r[0], 3.0);
        assert_f64_eq(r[1], -4.0);
        assert_f64_eq(r[2], 8.0);
    }

    #[test]
    fn divides_vector_by_scalar() {
        let v = Vec3::new(3.0, -6.0, 9.0);
        let r = v / 3.0;

        assert_f64_eq(r[0], 1.0);
        assert_f64_eq(r[1], -2.0);
        assert_f64_eq(r[2], 3.0);
    }

    #[test]
    fn add_assign_updates_vector() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v += Vec3::new(4.0, 5.0, 6.0);

        assert_f64_eq(v[0], 5.0);
        assert_f64_eq(v[1], 7.0);
        assert_f64_eq(v[2], 9.0);
    }

    #[test]
    fn mul_assign_updates_vector() {
        let mut v = Vec3::new(1.5, -2.0, 4.0);
        v *= 2.0;

        assert_f64_eq(v[0], 3.0);
        assert_f64_eq(v[1], -4.0);
        assert_f64_eq(v[2], 8.0);
    }

    #[test]
    fn div_assign_updates_vector() {
        let mut v = Vec3::new(3.0, -6.0, 9.0);
        v /= 3.0;

        assert_f64_eq(v[0], 1.0);
        assert_f64_eq(v[1], -2.0);
        assert_f64_eq(v[2], 3.0);
    }

    #[test]
    fn new_sets_components() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_f64_eq(v[0], 1.0);
        assert_f64_eq(v[1], 2.0);
        assert_f64_eq(v[2], 3.0);
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

        assert_f64_eq(c[0], -3.0);
        assert_f64_eq(c[1], 6.0);
        assert_f64_eq(c[2], -3.0);
    }

    #[test]
    fn unit_vector_has_length_one() {
        let v = Vec3::new(0.0, 3.0, 4.0);
        let u = Vec3::unit(v);

        assert_f64_eq(u.length(), 1.0);
    }
}

pub type Point3 = Vec3;
pub type Color3 = Vec3;
