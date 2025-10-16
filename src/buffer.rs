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

    pub fn push(&mut self, data: Color) {
        self.data.push(data);
    }

    #[allow(unused_attributes)]
    pub fn write(&mut self, index: usize, data: Color) {
        self.data[index] = data;
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
