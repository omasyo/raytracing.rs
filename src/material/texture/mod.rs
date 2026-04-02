use crate::color::Color;
use crate::perlin::Perlin;
use glam::Vec3;
use image::{open, Pixel, RgbImage};
use std::sync::Arc;

#[derive(Clone)]
pub enum TextureKind {
    SolidColor(Vec3),
    Checker {
        inv_scale: f32,
        even: Box<TextureKind>,
        odd: Box<TextureKind>,
    },
    Image(Arc<RgbImage>),
    Noise {
        noise: Perlin,
        scale: f32,
    },
}

impl TextureKind {
    pub fn solid(color: Vec3) -> Self {
        Self::SolidColor(color)
    }

    pub fn checker(scale: f32, c1: Vec3, c2: Vec3) -> Self {
        Self::Checker {
            inv_scale: scale.recip(),
            even: Box::new(Self::SolidColor(c1)),
            odd: Box::new(Self::SolidColor(c2)),
        }
    }

    pub fn checker_textured(scale: f32, even: Box<TextureKind>, odd: Box<TextureKind>) -> Self {
        Self::Checker {
            inv_scale: scale.recip(),
            even: even.clone(),
            odd: odd.clone(),
        }
    }

    pub fn image(file_path: &str) -> Self {
        Self::Image(Arc::new(
            open(file_path).expect("Cannot open file").into_rgb8(),
        ))
    }

    pub fn noise(scale: f32) -> Self {
        Self::Noise {
            noise: Perlin::new(),
            scale,
        }
    }

    pub fn value(&self, u: f32, v: f32, point: Vec3) -> Vec3 {
        match self {
            Self::SolidColor(color) => *color,
            Self::Checker {
                inv_scale,
                even,
                odd,
            } => {
                let x = (inv_scale * point.x).floor() as i32;
                let y = (inv_scale * point.y).floor() as i32;
                let z = (inv_scale * point.z).floor() as i32;
                if (x + y + z) % 2 == 0 {
                    even.value(u, v, point)
                } else {
                    odd.value(u, v, point)
                }
            }
            Self::Image(image) => {
                let i = (u * ((image.width() - 1) as f32)) as u32;
                let j = ((1.0 - v) * ((image.height() - 1) as f32)) as u32;
                let pixel = image.get_pixel(i, j).channels();
                let color = *Color::from_rgb(pixel[0], pixel[1], pixel[2]).vec3();
                color * color
            }
            Self::Noise { noise, scale } => {
                Vec3::splat(0.5)
                    * (1.0
                        + f32::sin(scale * point.z + 10.0 * noise.turbulence(point, 7)))
            }
        }
    }
}
