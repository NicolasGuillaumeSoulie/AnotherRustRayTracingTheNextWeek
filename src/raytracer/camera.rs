use super::ray::Ray;
use crate::raytracer::Hittable;
use crate::vec3::{Color, Point3, Vec3};
use rand::Rng;

pub struct Camera {
    img_width: u16,
    img_height: u16,

    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    pub fn new() -> Self {
        let img_aspect_ratio = 16.0 / 9.0;
        let img_width: u16 = 720;
        let img_height: u16 = (img_width * 9) / 16;

        let vp_height = 2.0;
        let vp_width = img_aspect_ratio * vp_height;
        let focal_len = 1.0;

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
        za_warudo: &dyn Hittable,
        samples_per_pixel: u16,
        max_depht: u16,
    ) -> String {
        let mut render = Vec::new();
        render.push(format!("P3\n{} {}\n255", self.img_width, self.img_height));

        let mut rng = rand::thread_rng();

        for j in (0..self.img_height).rev() {
            for i in 0..self.img_width {
                print!("\rScanlines remaining: {} ", j);
                let mut pixel_color = Color::zeros();
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + rng.gen_range(0.0..1.0)) / (self.img_width - 1) as f64;
                    let v = (j as f64 + rng.gen_range(0.0..1.0)) / (self.img_height - 1) as f64;

                    let r = self.get_ray(u, v);
                    pixel_color += r.color(&mut rng, za_warudo, max_depht);
                }

                render.push(pixel_color.write(samples_per_pixel));
            }
        }
        render.join("\n")
    }
}
