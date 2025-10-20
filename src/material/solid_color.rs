use crate::color::Color;
use glam::Vec3;
use crate::material::texture::Texture;

pub struct SolidColor {
    albedo: Vec3,
}

impl SolidColor {
    pub fn new(color: &Color) -> SolidColor {
        SolidColor {
            albedo: *color.vec3(),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _: f32, _: f32, _: Vec3) -> Vec3 {
        self.albedo
    }
}
