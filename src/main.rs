use raytracer::Camera;
use std::fs::File;
use std::io::prelude::*;

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
    let cam = Camera::new();
    let samples_per_pixel = 16;
    let max_depht = 16;

    let mat_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let mat_center = Lambertian::new(Color::new(0.8, 0.0, 0.8));
    let mat_lecft = Dielectric::new(1.5);
    let mat_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        mat_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        mat_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1., 0.0, -1.),
        0.5,
        mat_lecft,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        mat_right,
    )));

    let mut file = File::create("img.ppm")?;
    file.write_all(cam.render(&world, samples_per_pixel, max_depht).as_bytes())?;
    print!("\r### Rendering Done!! ###              ");
    Ok(())
}
