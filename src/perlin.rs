use glam::Vec3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    rand_float: [f32; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rand_float = [0.0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            rand_float[i] = rand::random::<f32>()
        }

        let mut perm_x = [0; POINT_COUNT];
        let mut perm_y = [0; POINT_COUNT];
        let mut perm_z = [0; POINT_COUNT];

        perlin_generate_perm(&mut perm_x);
        perlin_generate_perm(&mut perm_y);
        perlin_generate_perm(&mut perm_z);

        Self {
            rand_float,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Vec3) -> f32 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();

        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let ix = ((i + (di as i32)) & 255) as usize;
                    let iy = ((j + (dj as i32)) & 255) as usize;
                    let iz = ((k + (dk as i32)) & 255) as usize;

                    let idx = self.perm_x[ix] ^ self.perm_y[iy] ^ self.perm_z[iz];
                    c[di][dj][dk] = self.rand_float[idx];
                }
            }
        }

        trilinear_interpolation(&c, u, v, w)
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

pub fn trilinear_interpolation(c: &[[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let mut acc = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let c_val = c[i][j][k];
                let i = i as f32;
                let j = j as f32;
                let k = k as f32;
                acc += (i * u + (1.0 - i) * (1.0 - u))
                    * (j * v + (1.0 - j) * (1.0 - v))
                    * (k * w + (1.0 - k) * (1.0 - w))
                    * c_val;
            }
        }
    }
    acc
}
