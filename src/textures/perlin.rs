use rand::{rngs::ThreadRng, Rng};

use super::texture::Texture;
use crate::vec3::{dot, Color, Point3, Vec3};

pub struct Perlin {
    ran_vec: Vec<Vec3>,
    perm_x: Vec<u64>,
    perm_y: Vec<u64>,
    perm_z: Vec<u64>,
}

impl Perlin {
    const POINT_COUNT: u64 = 256;

    pub fn new(rng: &mut ThreadRng) -> Self {
        let mut ranfloat = Vec::with_capacity(Self::POINT_COUNT as usize);
        for _ in 0..Self::POINT_COUNT {
            ranfloat.push(Vec3::random_range(rng, -1.0, 1.0));
        }

        let perm_x = Self::perlin_generate_perm(rng);
        let perm_y = Self::perlin_generate_perm(rng);
        let perm_z = Self::perlin_generate_perm(rng);

        Self {
            ran_vec: ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        // NOTE: convert to i64 to prevent artefacts
        let i = p.x().floor() as i64;
        let j = p.y().floor() as i64;
        let k = p.z().floor() as i64;
        let mut c = [[[Vec3::new(); 2]; 2]; 2];

        for (di, i_arr) in c.iter_mut().enumerate() {
            for (dj, j_arr) in i_arr.iter_mut().enumerate() {
                for (dk, k_val) in j_arr.iter_mut().enumerate() {
                    let x = self.perm_x[((i + di as i64) & 255) as usize];
                    let y = self.perm_y[((j + dj as i64) & 255) as usize];
                    let z = self.perm_z[((k + dk as i64) & 255) as usize];
                    *k_val = self.ran_vec[(x ^ y ^ z) as usize];
                }
            }
        }

        Self::perlin_interp(&c, &u, &v, &w)
    }

    pub fn turb(&self, p: &Point3, depth: Option<u64>) -> f64 {
        let depth = depth.unwrap_or(7);

        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: &f64, v: &f64, w: &f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for (i, i_arr) in c.iter().enumerate() {
            for (j, j_arr) in i_arr.iter().enumerate() {
                for (k, k_val) in j_arr.iter().enumerate() {
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;
                    let weight = Vec3::with_values(u - i, v - j, w - k);
                    accum += (i * uu + (1.0 - i) * (1.0 - uu))
                        * (j * vv + (1.0 - j) * (1.0 - vv))
                        * (k * ww + (1.0 - k) * (1.0 - ww))
                        * dot(k_val, &weight);
                }
            }
        }
        accum
    }

    fn perlin_generate_perm(rng: &mut ThreadRng) -> Vec<u64> {
        let mut p = Vec::from_iter(0..Self::POINT_COUNT);
        Self::permute(rng, &mut p, Self::POINT_COUNT);
        p
    }

    fn permute(rng: &mut ThreadRng, p: &mut [u64], n: u64) {
        for i in (0..n).rev() {
            let target = rng.gen_range(0..=i) as usize;
            p.swap(i as usize, target);
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(rng: &mut ThreadRng, scale: f64) -> Self {
        Self {
            noise: Perlin::new(rng),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color::with_value(1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, None)).sin())
    }
}

#[macro_export]
macro_rules! rc_box_noise_texture {
    ($rng:expr, $scale:literal) => {
        Rc::new(Box::new(NoiseTexture::new($rng, $scale)))
    };
}
