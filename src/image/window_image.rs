use crate::buffer::{Buffer, DrawBuffer};
use softbuffer_quickstart::{SoftbufferWindow, WindowProperties};
use std::cmp::min;
use winit::event::WindowEvent;

pub struct WindowImage {
    window_name: String,
}

impl WindowImage {
    pub fn new(filename: &str) -> Self {
        Self {
            window_name: filename.to_string(),
        }
    }
}

impl DrawBuffer for WindowImage {
    fn draw_buffer(&self, buffer: &Buffer) {
        let properties = WindowProperties {
            width: buffer.width() as u32,
            height: buffer.height() as u32,
            title:  self.window_name.clone().leak(),
        };
        let buffer = buffer.clone();
        let mut window = SoftbufferWindow::new(properties);
        window
            .run(move |window, event| match event {
                WindowEvent::RedrawRequested => {
                    let (width, height) = window.inner_size();
                    let mut window_buffer = window.buffer_mut();
                    // for index in 0..(width * height) {
                    //     let y = index / width;
                    //     let x = index % width;
                    //     let red = (x * 255)/width;
                    //     let green = (y * 255)/height;
                    //     let blue = (255 - (red + green).min(255)) % 255;
                    //
                    //     buffer[index] = (blue | (green << 8) | (red << 16)).try_into().unwrap();
                    // }
                    for j in 0..buffer.height() {
                        for i in 0..buffer.width() {
                            let x = min(i, width-1);
                            let y = min(j, height-1);
                            window_buffer[(y * width) + x] = buffer.at((y * buffer.width()) + x).rgb_value();
                        }
                    }
                }
                _ => (),
            })
            .expect("window can't run :(");
    }
}
