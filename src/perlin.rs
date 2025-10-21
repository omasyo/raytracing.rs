use crate::utils::{random_vector, random_vector_range};
use glam::Vec3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    rand_vec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rand_vec = [Vec3::ZERO; POINT_COUNT];
        for i in 0..POINT_COUNT {
            rand_vec[i] = random_vector_range(-1.0, 1.0).normalize();
        }

        let mut perm_x = [0; POINT_COUNT];
        let mut perm_y = [0; POINT_COUNT];
        let mut perm_z = [0; POINT_COUNT];

        perlin_generate_perm(&mut perm_x);
        perlin_generate_perm(&mut perm_y);
        perlin_generate_perm(&mut perm_z);

        Self {
            rand_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Vec3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vec3::ZERO; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let ix = ((i + (di as i32)) & 255) as usize;
                    let iy = ((j + (dj as i32)) & 255) as usize;
                    let iz = ((k + (dk as i32)) & 255) as usize;

                    let idx = self.perm_x[ix] ^ self.perm_y[iy] ^ self.perm_z[iz];
                    c[di][dj][dk] = self.rand_vec[idx];
                }
            }
        }

        perlin_interpolation(&c, u, v, w)
    }

    pub fn turbulence(&self, p: Vec3, depth: i32) -> f32 {
        let mut acc = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        acc.abs()
    }
}

fn perlin_generate_perm(p: &mut [usize]) {
    for i in 0..POINT_COUNT {
        p[i] = i;
    }
    permute(p)
}

fn permute(p: &mut [usize]) {
    for i in (0..p.len()).rev() {
        let target = rand::random_range(0..=i);
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
}

pub fn perlin_interpolation(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut acc = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let c_val = c[i][j][k];
                let i = i as f32;
                let j = j as f32;
                let k = k as f32;
                let weight_v = Vec3::new(u - i, v - j, w - k);
                acc += (i * uu + (1.0 - i) * (1.0 - uu))
                    * (j * vv + (1.0 - j) * (1.0 - vv))
                    * (k * ww + (1.0 - k) * (1.0 - ww))
                    * c_val.dot(weight_v);
            }
        }
    }
    acc
}
