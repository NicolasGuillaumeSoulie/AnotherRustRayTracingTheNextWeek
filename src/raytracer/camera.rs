use std::sync::{Arc, Mutex};
use super::ray::Ray;
use crate::raytracer::Hittable;
use crate::vec3::{Color, Point3, Vec3};
use rand::{thread_rng, Rng};
use rayon::prelude::*;

pub struct Camera {
    img_width: u32,
    img_height: u32,

    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    pub fn new(img_aspect_ratio: f64, img_height: u32, focal_len: f64) -> Self {
        let img_width: u32 = ((img_height as f64) * img_aspect_ratio) as u32;

        let vp_height = 2.0;
        let vp_width = img_aspect_ratio * vp_height;

        let origin = Vec3::zeros();
        let horizontal = Vec3::new(vp_width, 0., 0.);
        let vertical = Vec3::new(0., vp_height, 0.);
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_len);

        Camera {
            img_width,
            img_height,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
    pub fn render(
        &self,
        za_warudo: &(dyn Hittable + Sync),
        samples_per_pixel: u16,
        max_depht: u16,
    ) -> String {
        let mut render = Vec::new();
        render.push(format!("P3\n{} {}\n255", self.img_width, self.img_height));

        // let mut rng = rand::thread_rng();

        // for j in (0..self.img_height).rev() {
        //     for i in 0..self.img_width {
        //         print!("\rScanlines remaining: {} ", j);
        //         let mut pixel_color = Color::zeros();
        //         for _ in 0..samples_per_pixel {
        //             let u = (i as f64 + rng.gen_range(0.0..1.0)) / (self.img_width - 1) as f64;
        //             let v = (j as f64 + rng.gen_range(0.0..1.0)) / (self.img_height - 1) as f64;

        //             let r = self.get_ray(u, v);
        //             pixel_color += r.color(&mut rng, za_warudo, max_depht);
        //         }

        //         render.push(pixel_color.write(samples_per_pixel));
        //     }
        // }
        // render.join("\n")
        let done = Arc::new(Mutex::new(0_u32));

        let image: String = (0..self.img_width * self.img_height)
            .into_par_iter()
            .map_init(
                || thread_rng(),
                |mut rng, screen_pos| {
                    let mut pixel_color = Color::zeros();
                    let i = screen_pos % self.img_width;
                    let j = self.img_height - 1 - screen_pos / self.img_width;
                    for _ in 0..samples_per_pixel {
                        let u = (i as f64 + rng.gen_range(0.0..1.0)) / (self.img_width - 1) as f64;
                        let v = (j as f64 + rng.gen_range(0.0..1.0)) / (self.img_height - 1) as f64;

                        let r = self.get_ray(u, v);
                        pixel_color += r.color(&mut rng, za_warudo, max_depht);
                    }

                    {
                        // Display progress
                        let mut lock = done.lock().unwrap();
                        *lock += 1;
                        print!(
                            "\rPixels done: {:>10}/{:<10} = {:>6.2}%",
                            *lock,
                            self.img_width * self.img_height,
                            (*lock as f64 / (self.img_width * self.img_height) as f64) * 100.0
                        );
                    }

                    pixel_color.write(samples_per_pixel) + "\n"
                },
            )
            .collect::<String>();

        render.push(image);
        render.join("\n")
    }
}
