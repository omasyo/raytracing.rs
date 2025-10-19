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
use crate::material::Material;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::utils::{random_vector, random_vector_range};
use crate::window::{SoftbufferWindow, WindowProperties};
use glam::vec3;
use std::sync::Arc;
use std::thread;
use winit::event::WindowEvent;

fn main() {
    let mut world: HittableList = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(vec3(0.5, 0.5, 0.5)));

    world.add(Arc::new(Sphere::new_stationary(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f32;
            let b = b as f32;
            let choose_mat = rand::random::<f32>();
            let center = vec3(
                a + 0.9 * rand::random::<f32>(),
                0.2,
                b + 0.9 * rand::random::<f32>(),
            );
    
            if (center - vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material + Sync>;
    
                if choose_mat < 0.8 {
                    let albedo = random_vector() * random_vector();
                    sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + vec3(0.0, rand::random_range(0.0..=0.5), 0.0);
                    world.add(Arc::new(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = random_vector_range(0.5, 1.0);
                    let fuzz = rand::random_range(0.0..=0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new_stationary(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else {
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new_stationary(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                };
            }
        }
    }
    
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new_stationary(
        vec3(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Arc::new(Lambertian::new(vec3(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new_stationary(
        vec3(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Arc::new(Metal::new(vec3(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new_stationary(
        vec3(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    
    let mut properties = CameraProperties::default();
    
    properties.aspect_ratio = 16.0 / 9.0;
    properties.image_width = 400;
    properties.samples_per_pixel = 100;
    properties.max_depth = 50;
    properties.v_fov = 20.0;
    properties.look_from = vec3(13.0, 2.0, 3.0);
    properties.look_at = vec3(0.0, 0.0, -1.0);
    properties.up = vec3(0.0, 1.0, 0.0);
    properties.defocus_angle = 0.6;
    properties.focus_dist = 10.0;
    
    let camera = Camera::new(properties);
    let mut buffer = Buffer::new(camera.image_width, camera.image_height);
    
    let properties = WindowProperties {
        width: camera.image_width as u32,
        height: camera.image_height as u32,
        title: "Haha",
    };
    // let buffer = buffer.clone();
    let mut window = SoftbufferWindow::new(properties);
    
    let (tx, rx) = std::sync::mpsc::channel::<Buffer>(); // row index, pixels
    thread::spawn(move || {
        camera.render(&world, tx);
    });
    
    window
        .run(move |window, event| {
            match event {
                WindowEvent::RedrawRequested => {
                    let (width, _height) = window.inner_size();
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
