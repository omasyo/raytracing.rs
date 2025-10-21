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
        Vec3::ONE * self.noise.turbulence(point, 7)
    }
}
