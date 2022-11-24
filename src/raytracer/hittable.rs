pub(crate) mod aabb;
pub(crate) mod material;
pub(crate) mod sphere;
use crate::raytracer::Ray;
use crate::vec3::{Point3, Vec3};
use material::Material;
pub use sphere::Sphere;
use std::sync::Arc;
use std::vec::Vec;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            p: Point3::zeros(),
            normal: Vec3::zeros(),
            t: 0.0,
            front_face: false,
            material: Material::None,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, out_normal: Vec3) {
        self.front_face = r.direction.dot(out_normal) < 0.0;
        self.normal = if self.front_face {
            out_normal
        } else {
            -out_normal
        };
    }
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
    // pub fn clear(&mut self) {
    //     self.objects.clear();
    // }
    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord { ..*rec };
        let mut hit_any = false;
        let mut closest = t_max;

        for obj in &self.objects {
            if obj.hit(r, t_min, closest, &mut temp_rec) {
                hit_any = true;
                closest = temp_rec.t;
                *rec = HitRecord { ..temp_rec };
            }
        }
        hit_any
    }
}

#[cfg(test)]
mod test {
    use super::HitRecord;
    use crate::{
        raytracer::Ray,
        vec3::{Point3, Vec3},
    };

    #[test]
    fn set_face_normal() {
        let mut rec = HitRecord::new();

        let origin = Point3::zeros();
        let direction = Vec3::new(1., 0., 0.);
        let ray = Ray::new(origin, direction, 0.0);

        let out_normal = Vec3::new(1., 0., 0.);

        rec.set_face_normal(&ray, out_normal);
        assert!(!rec.front_face);
        assert_eq!(rec.normal, -out_normal);

        rec.set_face_normal(&ray, -out_normal);
        assert!(rec.front_face);
        assert_eq!(rec.normal, -out_normal);
    }
}
