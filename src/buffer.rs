use glam::Vec3;
use crate::color::Color;


#[derive(Clone)]
pub struct Buffer {
    data: Vec<Color>,
    width: usize,
    height: usize,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: Vec::with_capacity(width * height),
            width,
            height,
        }
    }

    // pub fn push(&mut self, data: Color) {
    //     self.data.push(data);
    // }

    #[allow(unused_attributes)]
    pub fn write(&mut self, color: Vec3) {
        let color = Vec3 {
            x: linear_to_gamma(color.x),
            y: linear_to_gamma(color.y),
            z: linear_to_gamma(color.z),
        };
        self.data.push(Color::from_vec3(color));
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn at(&self, i: usize) -> &Color {
        &self.data[i]
    }
}

pub trait DrawBuffer {
    fn draw_buffer(&self, buffer: &Buffer);
}


pub fn linear_to_gamma(value: f32) -> f32 {
    if value > 0.0 {
        return value.sqrt();
    }
    0.0
}
