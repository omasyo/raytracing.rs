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
        let i = ((4.0 * p.x).floor() as i32 & 255) as usize;
        let j = ((4.0 * p.y).floor() as i32 & 255) as usize;
        let k = ((4.0 * p.z).floor() as i32 & 255) as usize;

        self.rand_float[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
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
