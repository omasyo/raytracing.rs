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
use crate::hittable::hittable_list::HittableList;
use crate::hittable::sphere::Sphere;
use crate::image::ppm_image::PpmImage;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::window::{SoftbufferWindow, WindowProperties};
use glam::vec3;
use std::sync::Arc;
use std::thread;
use winit::event::WindowEvent;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<Buffer>(); // row index, pixels

    let mut properties = CameraProperties::default();

    properties.aspect_ratio = 16.0 / 9.0;
    properties.image_width = 1200;
    properties.samples_per_pixel = 100;
    properties.max_depth = 50;
    properties.look_from = vec3(-2.0, 2.0, 1.0);
    properties.look_at = vec3(0.0, 0.0, -1.0);
    properties.up = vec3(0.0, 1.0, 0.0);
    properties.v_fov = 20.0;
    properties.defocus_angle = 10.0;
    properties.focus_dist = 3.4;

    let camera = Camera::new(properties);
    let mut buffer = Buffer::new(camera.image_width, camera.image_height);

    let properties = WindowProperties {
        width: camera.image_width as u32,
        height: camera.image_height as u32,
        title: "Haha",
    };
    // let buffer = buffer.clone();
    let mut window = SoftbufferWindow::new(properties);

    thread::spawn(move || {
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
        camera.render(&world, tx);
    });

    window
        .run(move |window, event| {
            match event {
                WindowEvent::RedrawRequested => {
                    let (width, height) = window.inner_size();
                    let mut window_buffer = window.buffer_mut();

                    if let Ok(b) = rx.recv() {
                        // unsafe {
                        //     buffer.write_at(index, pixel.clone());
                        // }
                        buffer = b.clone();
                        for (index, pixel) in b.data.iter().enumerate() {
                            let x = index % b.width;
                            let y = index / b.width;
                            let window_index = (y * width) + x;
                            window_buffer[window_index] = pixel.rgb_value();
                        }
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
