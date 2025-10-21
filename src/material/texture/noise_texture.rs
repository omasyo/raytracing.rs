use crate::material::texture::Texture;
use crate::perlin::Perlin;
use glam::Vec3;

pub struct NoiseTexture {
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f32, _: f32, point: Vec3) -> Vec3 {
        Vec3::ONE * self.noise.noise(point)
    }
}
