use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_fields() {
        let origin = Point3::new(0.0, 1.0, 0.0);
        let direction = Vec3::new(1.0, 2.0, 3.0);
        let ray = Ray::new(origin, direction);

        assert_eq!(ray.direction, direction);
        assert_eq!(ray.origin, origin);
    }

    #[test]
    fn ray_at_computes_point_along_ray() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(0.0, 1.0, 0.0);
        let r = Ray::new(origin, direction);

        // t = 0 should return the origin
        assert_eq!(r.at(0.0), origin);

        // at t = 2.0 we should be two units along the direction in Y
        assert_eq!(r.at(2.0), Point3::new(1.0, 4.0, 3.0));
    }
}
