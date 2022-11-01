use super::HitRecord;
use crate::{
    raytracer::Ray,
    vec3::{Color, Vec3},
};
use rand::{rngs::ThreadRng, Rng};

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    None,
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, rng: &mut ThreadRng) -> (Vec3, Ray) {
        match self {
            Material::Lambertian(l) => l.scatter(rec, rng),
            Material::Metal(m) => m.scatter(r_in, rec, rng),
            Material::Dielectric(d) => d.scatter(r_in, rec, rng),
            _ => (Color::zeros(), Ray::new(Vec3::zeros(), Vec3::zeros())),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Material {
        Material::Lambertian(Lambertian { albedo })
    }
    fn scatter(&self, rec: &mut HitRecord, rng: &mut ThreadRng) -> (Vec3, Ray) {
        let mut direction = rec.normal + Vec3::rand_unit(rng);

        if direction.near_zero() {
            direction = rec.normal;
        }
        (self.albedo, Ray::new(rec.p, direction))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Material {
        Material::Metal(Metal { albedo, fuzz })
    }
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, rng: &mut ThreadRng) -> (Vec3, Ray) {
        let reflected = r_in.direction.normalize().reflect(rec.normal);
        (
            self.albedo,
            Ray::new(rec.p, reflected + self.fuzz * Vec3::rand_in_sphere(rng)),
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Material {
        Material::Dielectric(Dielectric { ir })
    }
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, rng: &mut ThreadRng) -> (Vec3, Ray) {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let direction_norm = r_in.direction.normalize();
        let cos_theta = (-direction_norm).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
        {
            let reflect = direction_norm.reflect(rec.normal);
            (Color::ones(), Ray::new(rec.p, reflect))
        } else {
            let refracted = direction_norm.refract(rec.normal, refraction_ratio);
            (Color::ones(), Ray::new(rec.p, refracted))
        }
    }
    fn reflectance(cosine: f64, refraction_ratio: f64) -> f64 {
        // Schlick's approximation
        let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
