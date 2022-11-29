use super::{HitRecord, Hittable};
use crate::raytracer::Ray;
use crate::vec3::Point3;
use std::sync::Arc;

// Axis-Aligned Bounding Box
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Aabb {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl Aabb {
    pub fn new(minimum: Point3, maximum: Point3) -> Self {
        Aabb { minimum, maximum }
    }
    pub fn surronding_box(box_a: &Aabb, box_b: &Aabb) -> Aabb {
        let minimum = Point3::new(
            box_a.minimum.x.min(box_b.minimum.x),
            box_a.minimum.y.min(box_b.minimum.y),
            box_a.minimum.z.min(box_b.minimum.z),
        );

        let maximum = Point3::new(
            box_a.maximum.x.max(box_b.maximum.x),
            box_a.maximum.y.max(box_b.maximum.y),
            box_a.maximum.z.max(box_b.maximum.z),
        );

        Aabb { minimum, maximum }
    }
}

impl Hittable for Aabb {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, _rec: &mut HitRecord) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let mut t0 = (self.minimum[a] - r.origin[a]) * inv_d;
            let mut t1 = (self.maximum[a] - r.origin[a]) * inv_d;
            if inv_d < 0.0 {
                (t0, t1) = (t1, t0);
            }
            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    fn bounding_box(&self, time_frame: (f64, f64)) -> Option<Self> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Aabb;
    use crate::{
        raytracer::{HitRecord, Hittable, Ray},
        vec3::Point3,
    };

    #[test]
    fn hit_me() {
        let aabb = Aabb {
            minimum: Point3::new(1., 1., 1.),
            maximum: Point3::new(2., 2., 2.),
        };

        let ray_ok = Ray::new(Point3::zeros(), Point3::new(1., 1., 1.), 0.);

        let ray_not_ok = Ray::new(Point3::zeros(), Point3::up(), 0.);

        assert!(aabb.hit(
            &ray_ok,
            f64::MIN_POSITIVE,
            f64::INFINITY,
            &mut HitRecord::new()
        ));
        assert!(!aabb.hit(
            &ray_not_ok,
            f64::MIN_POSITIVE,
            f64::INFINITY,
            &mut HitRecord::new()
        ));
    }
}
