use rand::{thread_rng, Rng};
use raytracer::Camera;
use std::env;
use std::io::prelude::*;
use std::{fs::File, sync::Arc};

use crate::{
    raytracer::{
        hittable::material::{Dielectric, Lambertian, Metal},
        HittableList, Sphere,
    },
    vec3::{Color, Point3, Vec3},
};
mod raytracer;
mod vec3;

fn main() -> std::io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0., 0., 0.);
    let dist_to_focus = (lookfrom - lookat).length();

    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::up(),
        16. / 9.,
        240,
        20.,
        0.1,
        dist_to_focus,
    );
    let samples_per_pixel = 128;
    let max_depht = 16;

    let mat_ground = Lambertian::new(Color::new(0.5, 0.5, 0.4));
    let mat_lambertian = Lambertian::new(Color::new(0.8, 0.0, 0.8));
    let mat_dielectric = Dielectric::new(1.5);
    let mat_metal = Metal::new(Color::new(0.7, 0.6, 0.5), 0.01);

    let mut world = random_scene();
    world.add(Arc::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.0,
        mat_lambertian,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., -1.),
        1000.,
        mat_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.0,
        mat_dielectric,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.0,
        mat_metal,
    )));

    let mut file = File::create("img.ppm")?;
    let content = cam.render(&world, samples_per_pixel, max_depht, 0.0..1.0);
    file.write_all(content.as_bytes())?;
    print!("\n### Rendering Done!! ###              ");
    Ok(())
}

fn random_scene() -> HittableList {
    let mut za_warudo = HittableList::new();
    let mut rng = thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let chose_mat = rng.gen_range(0..100);
            let center = Point3 {
                x: a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                y: 0.2,
                z: b as f64 + 0.9 * rng.gen_range(0.0..1.0),
            };

            match chose_mat {
                // Diffuse
                0..=79 => za_warudo.add(Arc::new(Sphere::new_moving(
                    center,
                    Vec3::up() * rng.gen_range(0.0..0.5),
                    0.2,
                    Lambertian::new(
                        Color::rand(&mut rng, 0.0, 1.0) * Color::rand(&mut rng, 0.0, 1.0),
                    ),
                ))),
                // Metal
                80..=94 => za_warudo.add(Arc::new(Sphere::new(
                    center,
                    0.2,
                    Metal::new(Color::rand(&mut rng, 0.5, 1.0), rng.gen_range(0.0..0.5)),
                ))),
                // Glass
                _ => za_warudo.add(Arc::new(Sphere::new(center, 0.2, Dielectric::new(1.5)))),
            }
        }
    }

    za_warudo
}
