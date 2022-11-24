use super::{material::Material, HitRecord, Hittable};
use crate::{raytracer::Ray, vec3::{Vec3, Point3}};

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
        self.center + time*self.speed
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

        // let in_sphere = (r.origin - self.center).length() <= self.radius && (rec.p - self.center).length() <= self.radius;
        // match self.material{
        //     Material::Dielectric(d) => {
        //         println!("In sphere : {}", in_sphere)
        //     },
        //     _ => (),
        // }

        true
    }
}
