use raytracer::Camera;
use std::env;
use std::f64::consts::PI;
use std::io::prelude::*;
use std::{fs::File, sync::Arc};

use crate::{
    raytracer::{
        hittable::material::{Dielectric, Lambertian, Metal},
        HittableList, Sphere,
    },
    vec3::{Color, Point3},
};
mod raytracer;
mod vec3;

fn main() -> std::io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let cam = Camera::new(16. / 9., 405, 90., 1.0);
    let samples_per_pixel = 128;
    let max_depht = 50;

    let mat_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let mat_center = Lambertian::new(Color::new(0.8, 0.0, 0.8));
    let mat_lecft = Dielectric::new(1.5);
    let mat_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.1);

    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        mat_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        mat_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1., 0.0, -1.),
        0.5,
        mat_lecft,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        mat_right,
    )));

    let mut file = File::create("img.ppm")?;
    let content = cam.render(&world, samples_per_pixel, max_depht);
    file.write_all(content.as_bytes())?;
    print!("\n### Rendering Done!! ###              ");
    Ok(())
}
