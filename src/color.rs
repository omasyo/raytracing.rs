use glam::Vec3;
use std::ops::{Add, AddAssign, Mul};

#[derive(Clone)]
pub struct Color {
    value: Vec3,
}

impl Color {
    pub fn new(value: Vec3) -> Color {
        Color { value }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            value: Vec3 {
                x: r as f32 / 255.0,
                y: g as f32 / 255.0,
                z: b as f32 / 255.0,
            },
        }
    }

    pub fn vec3(&self) -> &Vec3 {
        &self.value
    }

    #[allow(unused_attributes)]
    pub fn red(&self) -> u8 {
        (self.value.x * 255.0).round() as u8
    }
    pub fn green(&self) -> u8 {
        (self.value.y * 255.0).round() as u8
    }
    pub fn blue(&self) -> u8 {
        (self.value.z * 255.0).round() as u8
    }

    pub fn rgb(&self) -> (u8, u8, u8) {
        (self.red(), self.green(), self.blue())
    }

    pub fn rgb_value(&self) -> u32 {
        // let mut color: u32 = 0;
        // color |= ((v.x * 255f32) as u32) << 16;
        // color |= ((v.y * 255f32) as u32) << 8;
        // color |= (v.z * 255f32) as u32;
        // Color { value: color }
        // self.value & 0x00FFFFFF

        let v = self.value;

        let r = self.red() as u32;
        let g = self.green() as u32;
        let b = self.blue() as u32;
        (r << 16) | (g << 8) | b
    }

    // pub fn linear_to_gamma(&self) -> Self {
    //     Color::new(
    //         self.red().isqrt(),
    //         self.green().isqrt(),
    //         self.blue().isqrt(),
    //     )
    // }
}


pub fn linear_to_gamma(value: f32) -> f32 {
    if value > 0.0 {
        return value.sqrt();
    }
    0.0
}
