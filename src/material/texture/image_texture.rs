use crate::material::texture::Texture;
use glam::Vec3;
use image::{open, Pixel, RgbImage};
use crate::color::Color;

pub struct ImageTexture {
    image: RgbImage,
}

impl ImageTexture {
    pub fn new(file_path: &str) -> ImageTexture {
        Self {
            image: open(file_path).expect("Cannot open file").into_rgb8(),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _: Vec3) -> Vec3 {
        assert!(u >= 0.0 && u <= 1.0 && v >= 0.0 && v <= 1.0, "{u} {v}");

        let i = (u * ((self.image.width() - 1) as f32)) as u32;
        let j = ((1.0 - v) * ((self.image.height() - 1) as f32)) as u32;
        let pixel = self.image.get_pixel(i, j).channels();

        let color = *Color::from_rgb(pixel[0], pixel[1], pixel[2]).vec3();
        color * color // remove gamma correction I think
    }
}
