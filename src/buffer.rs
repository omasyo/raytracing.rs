use crate::color::Color;

#[derive(Clone)]
pub struct Buffer {
    pub data: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![Color::new(0, 0, 0); width * height], //Vec::with_capacity(width * height),
            width,
            height,
        }
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
