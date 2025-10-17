use crate::buffer::{Buffer, DrawBuffer};
use crate::window::{SoftbufferWindow, WindowProperties};
use std::cmp::min;
use std::thread;
use std::time::Duration;
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
            title: self.window_name.clone().leak(),
        };
        let buffer = buffer.clone();
        let mut window = SoftbufferWindow::new(properties);
        window
            .run(move |window, event| {
                // match event {
                //     // WindowEvent::RedrawRequested => {
                //     //
                //     // }
                //     _ => {}
                // }
                let (width, height) = window.inner_size();
                let mut window_buffer = window.buffer_mut();
                for j in 0..buffer.height() {
                    for i in 0..buffer.width() {
                        let x = min(i, width - 1);
                        let y = min(j, height - 1);
                        window_buffer[(y * width) + x] =
                            buffer.at((y * buffer.width()) + x).rgb_value();
                    }
                }
            })
            .expect("window can't run :(");
    }
}
