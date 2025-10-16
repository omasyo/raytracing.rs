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

        // for index in 0..(width * height) {
        //     let y = index / width;
        //     let x = index % width;
        //     let red = (x * 255)/width;
        //     let green = (y * 255)/height;
        //     let blue = (255 - (red + green).min(255)) % 255;
        //
        //     buffer[index] = (blue | (green << 8) | (red << 16)).try_into().unwrap();
        // }
        for y in 0..height {
            for x in 0..width {
                let index = y * width + x;
                let pixel = buffer.at(index);
                let red = pixel.red();
                let green = pixel.green();
                let blue = pixel.blue();

                write!(out, "{red} {green} {blue}\n").unwrap();
            }
        }
    }
}
