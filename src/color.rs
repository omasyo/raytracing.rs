use std::ops::{Add, AddAssign, Mul};
use glam::Vec3;

#[derive(Clone)]
pub struct Color {
    value: u32,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        let mut color: u32 = 0;
        color |= (r as u32) << 16;
        color |= (g as u32) << 8;
        color |= b as u32;
        Color { value: color }
    }

    pub fn from_vec3(v: Vec3) -> Color {
        let mut color: u32 = 0;
        color |= ((v.x * 255f32) as u32) << 16;
        color |= ((v.y * 255f32) as u32) << 8;
        color |= (v.z * 255f32) as u32;
        Color { value: color }
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.red() as f32 / 255.0,
            y: self.green() as f32 / 255.0,
            z: self.blue() as f32 / 255.0,
        }
    }

    #[allow(unused_attributes)]
    pub fn red(&self) -> u8 {
        ((self.value >> 16) & 0xFF) as u8
    }
    pub fn green(&self) -> u8 {
        ((self.value >> 8) & 0xFF) as u8
    }
    pub fn blue(&self) -> u8 {
        (self.value & 0xFF) as u8
    }

    pub fn rgb(&self) -> (u8, u8, u8) {
        (self.red(), self.green(), self.blue())
    }

    pub fn rgb_value(&self) -> u32 {
        self.value & 0x00FFFFFF
    }

   pub fn linear_to_gamma(&self) -> Self {
        Color::new(
            self.red().isqrt(),
            self.green().isqrt(),
            self.blue().isqrt(),
        )
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    
    fn mul(self, other: f64) -> Self::Output {
        Color::new(
            ((self.red() as f64) * other) as u8,
            ((self.green() as f64) * other) as u8,
            ((self.blue() as f64) * other) as u8,
        )
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(self.red() + rhs.red(), self.green() + rhs.green(), self.blue() + rhs.blue())
    }
}