use crate::material::texture::Texture;
use crate::perlin::Perlin;
use glam::Vec3;

pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f32, _: f32, point: Vec3) -> Vec3 {
        Vec3::ONE * 0.5 * (1.0 + self.noise.noise(point * self.scale))
    }
}
