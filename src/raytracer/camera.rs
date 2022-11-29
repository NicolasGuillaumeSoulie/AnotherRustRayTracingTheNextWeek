use super::ray::Ray;
use crate::raytracer::Hittable;
use crate::vec3::{Color, Point3, Vec3};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::ops::Range;
use std::sync::{Arc, Mutex};

pub struct Camera {
    img_width: u32,
    img_height: u32,

    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    u: Vec3,
    v: Vec3,
    // w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        img_aspect_ratio: f64,
        img_height: u32,
        vfov: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let img_width: u32 = ((img_height as f64) * img_aspect_ratio) as u32;

        let h = (0.5 * vfov.to_radians()).tan();
        let vp_height = h * 2.0;
        let vp_width = img_aspect_ratio * vp_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_distance * vp_width * u;
        let vertical = focus_distance * vp_height * v;
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - focus_distance * w;

        let lens_radius = 0.5 * aperture;

        Camera {
            img_width,
            img_height,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            // w,
            lens_radius,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64, rng: &mut ThreadRng, time_frame: &Range<f64>) -> Ray {
        let rd = self.lens_radius * Vec3::rand_in_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
            rng.gen_range(time_frame.clone()),
        )
    }
    pub fn render(
        &self,
        za_warudo: &(dyn Hittable + Sync),
        samples_per_pixel: u16,
        max_depht: u16,
        time_frame: Range<f64>,
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
            .map_init(thread_rng, |rng, screen_pos| {
                let mut pixel_color = Color::zeros();
                let i = screen_pos % self.img_width;
                let j = self.img_height - 1 - screen_pos / self.img_width;
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + rng.gen_range(0.0..1.0)) / (self.img_width - 1) as f64;
                    let v = (j as f64 + rng.gen_range(0.0..1.0)) / (self.img_height - 1) as f64;

                    let r = self.get_ray(u, v, rng, &time_frame);
                    pixel_color += r.color(rng, za_warudo, max_depht);
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
            })
            .collect::<String>();

        render.push(image);
        render.join("\n")
    }
}
