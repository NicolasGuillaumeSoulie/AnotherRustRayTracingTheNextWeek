use crate::raytracer::{HitRecord, Hittable};
use crate::vec3::{Color, Point3, Vec3};
use rand::rngs::ThreadRng;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
    pub fn color(&self, rng: &mut ThreadRng, za_warudo: &dyn Hittable, depht: u16) -> Color {
        if depht < 1 {
            return Color::zeros();
        }
        // Object intersection
        let mut rec = HitRecord::new();
        if za_warudo.hit(self, 0.001, f64::INFINITY, &mut rec) {
            let material = rec.material;
            match material.scatter(self, &mut rec, rng) {
                Option::Some((attenuation, new_ray)) => {
                    return attenuation * new_ray.color(rng, za_warudo, depht - 1)
                }
                _ => return Color::zeros(),
            }
        }
        // Background
        let norm_dir = self.direction.normalize();
        let t = 0.5 * (norm_dir.y + 1.);
        (1.0 - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
    }
}
