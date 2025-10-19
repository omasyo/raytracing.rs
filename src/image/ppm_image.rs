use crate::buffer::{Buffer, DrawBuffer};
use std::fs::File;
use std::io::{self, Write};

pub struct PpmImage {
    filename: String,
}

impl PpmImage {
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
        }
    }
}

impl DrawBuffer for PpmImage {
    fn draw_buffer(&self, buffer: &Buffer) {
        let width = buffer.width();
        let height = buffer.height();

        let file = File::create(&self.filename).unwrap();
        let mut out = io::BufWriter::new(file);
        write!(out, "P3\n{width} {height}\n255\n").unwrap();

        for index in 0..(width * height) {
            let pixel = buffer.at(index);
            let red = pixel.red();
            let green = pixel.green();
            let blue = pixel.blue();

            write!(out, "{red} {green} {blue}\n").unwrap();
        }
    }
}
