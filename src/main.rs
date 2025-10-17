mod buffer;
mod camera;
mod color;
mod hittable;
mod image;
mod interval;
mod material;
mod ray;
mod utils;
mod window;

use crate::buffer::{Buffer, DrawBuffer};
use crate::camera::{Camera, CameraProperties};
use crate::color::Color;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::sphere::Sphere;
use crate::image::ppm_image::PpmImage;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::window::{SoftbufferWindow, WindowProperties};
use glam::vec3;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::thread;
use winit::event::WindowEvent;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<(usize, Color)>(); // row index, pixels

    thread::spawn(move || {
        let mut local = Buffer::new(400, 25 * 9);

        let mut world: HittableList = HittableList::new();

        let material_ground = Arc::new(Lambertian::new(vec3(0.8, 0.8, 0.0)));
        let material_center = Arc::new(Lambertian::new(vec3(0.1, 0.2, 0.5)));
        let material_left = Arc::new(Dielectric::new(1.5));
        let material_bubble = Arc::new(Dielectric::new(1.0 / 1.5));
        let material_right = Arc::new(Metal::new(vec3(0.8, 0.6, 0.2), 1.0));

        world.add(Box::new(Sphere::new(
            vec3(0.0, -100.5, -1.0),
            100.0,
            material_ground,
        )));
        world.add(Box::new(Sphere::new(
            vec3(0.0, 0.0, -1.2),
            0.5,
            material_center,
        )));
        world.add(Box::new(Sphere::new(
            vec3(-1.0, 0.0, -1.0),
            0.5,
            material_left,
        )));
        world.add(Box::new(Sphere::new(
            vec3(-1.0, 0.0, -1.0),
            0.4,
            material_bubble,
        )));
        world.add(Box::new(Sphere::new(
            vec3(1.0, 0.0, -1.0),
            0.5,
            material_right,
        )));

        let properties = CameraProperties::default()
            .set_aspect_ratio(16.0 / 9.0)
            .set_image_width(400)
            .set_samples_per_pixel(100)
            .set_max_depth(50);

        let camera = Camera::new(properties);
        camera.render(&world, tx);
    });

    let mut buffer = Buffer::new(400, 25 * 9);

    let properties = WindowProperties {
        width: 400,
        height: 25 * 9,
        title: "Haha",
    };
    // let buffer = buffer.clone();
    let mut window = SoftbufferWindow::new(properties);
    window
        .run(move |window, event| {
            match event {
                WindowEvent::RedrawRequested => {
                    let mut window_buffer = window.buffer_mut();
                    if let Ok((index, pixel)) = rx.recv() {
                        unsafe {
                            buffer.write_at(index, pixel.clone());
                        }
                        window_buffer[index] = pixel.rgb_value();
                    }
                }
                WindowEvent::CloseRequested => {
                    let image = PpmImage::new("image.ppm");
                    image.draw_buffer(&buffer);
                }
                _ => {}
            }
        })
        .expect("window can't run :(");
}
