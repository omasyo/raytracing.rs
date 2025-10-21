use crate::color::Color;
use crate::material::solid_color::SolidColor;
use crate::material::texture::Texture;
use crate::material::Material;
use glam::Vec3;

pub struct DiffuseLight {
    texture: Box<dyn Texture>,
}

impl From<Vec3> for DiffuseLight {
    fn from(color: Vec3) -> Self {
        Self {
            texture: Box::new(SolidColor::new(&Color::new(color))),
        }
    }
}

impl From<&Color> for DiffuseLight {
    fn from(color: &Color) -> Self {
        Self {
            texture: Box::new(SolidColor::new(color)),
        }
    }
}

impl From<Box<dyn Texture>> for DiffuseLight {
    fn from(texture: Box<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.texture.value(u, v, p)
    }
}
