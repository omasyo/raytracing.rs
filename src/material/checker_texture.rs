use glam::Vec3;
use crate::color::Color;
use crate::material::solid_color::SolidColor;
use crate::material::texture::Texture;

pub struct CheckerTexture {
    inverse_scale: f32,
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f32, even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        Self {
            inverse_scale: scale.recip(),
            even,
            odd,
        }
    }

    pub fn from_color(scale: f32, c1: &Color, c2: &Color) -> Self {
        Self {
            inverse_scale: scale.recip(),
            even: Box::new(SolidColor::new(c1)),
            odd: Box::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, point: Vec3) -> Vec3 {
        let x = (self.inverse_scale * point.x).floor() as i32;
        let y = (self.inverse_scale * point.y).floor() as i32;
        let z = (self.inverse_scale * point.z).floor() as i32;

        let is_even = (x+y+z) % 2 == 0;

       if is_even { self.even.value(u, v, point) } else {self.odd.value(u, v, point)}
    }
}