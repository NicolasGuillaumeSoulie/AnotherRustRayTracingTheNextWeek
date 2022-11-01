use rand::{rngs::ThreadRng, Rng};
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
    pub fn dot(&self, v: Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
    pub fn cross(&self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self[1] * v[2] - self[2] * v[1],
            y: self[2] * v[0] - self[0] * v[2],
            z: self[0] * v[1] - self[1] * v[0],
        }
    }
    pub fn near_zero(&self) -> bool {
        let delta = 1e-8;
        self.x.abs() < delta && self.y.abs() < delta && self.z.abs() < delta
    }
    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        *self - 2.0 * self.dot(normal) * normal
    }
    pub fn refract(&self, normal: Vec3, etai_over_etai: f64) -> Vec3 {
        let cos_theta = 1.0_f64.min(normal.dot(-*self));
        let r_perpendicular_norm = etai_over_etai * (cos_theta * normal + *self);
        let r_parallel_norm =
            -((1.0 - r_perpendicular_norm.length_squared()).abs().sqrt()) * normal;

        r_perpendicular_norm + r_parallel_norm
    }
    // Vec3 basic generation
    pub fn zeros() -> Vec3 {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
    pub fn ones() -> Vec3 {
        Self {
            x: 1.,
            y: 1.,
            z: 1.,
        }
    }
    // Vec3 rng generation
    pub fn rand(rng: &mut ThreadRng, min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }
    pub fn rand_in_sphere(rng: &mut ThreadRng) -> Vec3 {
        loop {
            let p = Vec3::rand(rng, -1., 1.);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
    pub fn rand_unit(rng: &mut ThreadRng) -> Vec3 {
        Self::rand_in_sphere(rng).normalize()
    }
}
// vec3 . float operations
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}
impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

// . vec3 operations
impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl ops::Index<u64> for Vec3 {
    type Output = f64;

    fn index(&self, index: u64) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Out of range vec3 index!"),
        }
    }
}
impl From<(f64, f64, f64)> for Vec3 {
    fn from(v: (f64, f64, f64)) -> Self {
        Self {
            x: v.0,
            y: v.1,
            z: v.2,
        }
    }
}

// vec3 . vec3 operations
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl Color {
    pub fn write(&self, samples_per_pixel: u16) -> String {
        let scale = 1.0 / samples_per_pixel as f64;

        let r = (scale * self.x).sqrt();
        let g = (scale * self.y).sqrt();
        let b = (scale * self.z).sqrt();

        format!(
            "{} {} {}",
            (255.99 * r.clamp(0.0, 0.999)) as u16,
            (255.99 * g.clamp(0.0, 0.999)) as u16,
            (255.99 * b.clamp(0.0, 0.999)) as u16
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    #[test]
    fn normalize() {
        let v1 = Vec3::new(2.0, 2.0, 2.0);
        let v2 = Vec3::new(1.0, 1.0, 1.0);
        let v3 = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(v1.normalize(), v2.normalize());
        assert_ne!(v1.normalize(), v3.normalize());
        assert_eq!(v1.normalize().length(), 1.0);
        assert_eq!(v2.normalize().length(), 1.0);
        assert_eq!(v3.normalize().length(), 1.0);
    }

    #[test]
    fn dot_product() {
        let u = Vec3::new(1., 2., 3.);
        let v = Vec3::new(4., -5., 6.);
        assert_eq!(u.dot(v), 12.);

        let u = Vec3::new(-4., -9., 0.);
        let v = Vec3::new(-1., 2., 0.);
        assert_eq!(u.dot(v), -14.);
        // check for mutation
        assert_eq!(u, Vec3::new(-4., -9., 0.));
        assert_eq!(v, Vec3::new(-1., 2., 0.));
    }

    #[test]
    fn cross_product() {
        let u = Vec3::from((3., -3., 1.));
        let v = Vec3::from((4., 9., 2.));
        let cross = Vec3::from((-15., -2., 39.));
        assert_eq!(u.cross(v), cross);

        let v = Vec3::from((-12., 12., -4.));
        let cross = Vec3::from((0., 0., 0.));
        assert_eq!(u.cross(v), cross);
    }

    #[test]
    fn add() {
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 2.0
            } + Vec3 {
                x: 2.0,
                y: 1.0,
                z: 2.0
            },
            Vec3 {
                x: 3.0,
                y: 1.0,
                z: 4.0
            }
        );
    }

    #[test]
    fn add_assign() {
        let mut x = Vec3::new(0.0, 0.0, 0.0);
        let y = Vec3::new(1.0, 2.0, 3.0);
        x += y;
        assert_eq!(
            x,
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn cross() {
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 2.0
            }
            .cross(Vec3 {
                x: 2.0,
                y: 1.0,
                z: 2.0
            }),
            Vec3 {
                x: -2.0,
                y: 2.0,
                z: 1.0
            }
        );
    }

    #[test]
    fn dot() {
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 2.0
            }
            .dot(Vec3 {
                x: 2.0,
                y: 1.0,
                z: 2.0
            }),
            6.0
        );
    }

    #[test]
    fn length() {
        let v = Vec3 {
            x: -2.0,
            y: -2.0,
            z: -1.0,
        };
        let u = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        assert_eq!(v.length(), 3.0);
        assert_eq!(u.length(), 1.0);
    }

    #[test]
    fn squared_length() {
        let v = Vec3 {
            x: -2.0,
            y: -2.0,
            z: -1.0,
        };
        let u = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        assert_eq!(v.length_squared(), 9.0);
        assert_eq!(u.length_squared(), 1.0);
    }

    #[test]
    fn mul() {
        assert_eq!(
            3.0 * Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            },
            Vec3 {
                x: 3.0,
                y: 6.0,
                z: 9.0
            }
        );
    }

    #[test]
    fn hadamard() {
        let lhs = Vec3::new(1.0, 1.0, 1.0);
        let rhs = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(lhs * rhs, Vec3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn neg() {
        assert_eq!(
            -Vec3 {
                x: 1.0,
                y: -2.0,
                z: 3.0
            },
            Vec3 {
                x: -1.0,
                y: 2.0,
                z: -3.0
            }
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 2.0
            } - Vec3 {
                x: 2.0,
                y: 1.0,
                z: 2.0
            },
            Vec3 {
                x: -1.0,
                y: -1.0,
                z: 0.0
            }
        );
    }
}
