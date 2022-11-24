use super::{material::Material, HitRecord, Hittable, AABB};
use crate::{
    raytracer::Ray,
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    center: Point3,
    speed: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Sphere {
            center,
            speed: Vec3::zeros(),
            radius,
            material,
        }
    }
    pub fn new_moving(center: Point3, speed: Vec3, radius: f64, material: Material) -> Self {
        Sphere {
            center,
            speed,
            radius,
            material,
        }
    }
    pub fn center(&self, time: f64) -> Point3 {
        self.center + time * self.speed
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center(r.time);
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let out_normal = (rec.p - self.center(r.time)) / self.radius;
        rec.set_face_normal(r, out_normal);
        rec.material = self.material;

        true
    }

    fn bounding_box(&self, time_frame: (f64, f64)) -> Option<super::AABB> {
        let box_a = AABB::new(
            self.center(time_frame.0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time_frame.0) + Vec3::new(self.radius, self.radius, self.radius),
        );

        let box_b = AABB::new(
            self.center(time_frame.1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time_frame.1) + Vec3::new(self.radius, self.radius, self.radius),
        );

        Option::Some(AABB::surronding_box(&box_a, &box_b))
    }
}

#[cfg(test)]
mod tests {
    use super::Sphere;
    use crate::{
        raytracer::{
            hittable::{material::Material, AABB},
            Hittable,
        },
        vec3::Vec3,
    };

    #[test]
    fn bounding_box() {
        let sphere = Sphere::new(Vec3::zeros(), 1.0, Material::None);
        let expected_bounding_box = AABB::new(-Vec3::ones(), Vec3::ones());

        assert_eq!(
            expected_bounding_box,
            sphere.bounding_box((0.0, 0.0)).unwrap()
        );
    }

    #[test]
    fn bounding_box_moving() {
        let sphere = Sphere::new_moving(Vec3::zeros(), Vec3::up(), 1.0, Material::None);
        let expected_bounding_box = AABB::new(-Vec3::ones(), Vec3::ones() + Vec3::up());

        assert_eq!(
            expected_bounding_box,
            sphere.bounding_box((0.0, 1.0)).unwrap()
        );
    }
}
