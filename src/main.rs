mod buffer;
mod camera;
mod color;
mod hittable;
mod image;
mod interval;
mod material;
mod perlin;
mod ray;
mod utils;
mod window;

use crate::buffer::{Buffer, DrawBuffer};
use crate::camera::{Camera, CameraProperties};
use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable::bvh::BvhNode;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::quad::Quad;
use crate::hittable::sphere::Sphere;
use crate::image::ppm_image::PpmImage;
use crate::material::Material;
use crate::material::dielectric::Dielectric;
use crate::material::diffuse_light::DiffuseLight;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::material::texture::Texture;
use crate::material::texture::checker_texture::CheckerTexture;
use crate::material::texture::image_texture::ImageTexture;
use crate::material::texture::noise_texture::NoiseTexture;
use crate::utils::{random_vector, random_vector_range};
use crate::window::{SoftbufferWindow, WindowProperties};
use glam::{Vec3, vec3};
use std::sync::Arc;
use std::thread;
use winit::event::WindowEvent;

fn main() {
    let (world, camera) = match 6 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        _ => {
            unreachable!()
        }
    };

    let mut buffer = Buffer::new(camera.image_width, camera.image_height);

    let properties = WindowProperties {
        width: camera.image_width as u32,
        height: camera.image_height as u32,
        title: "Haha",
    };

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
                            window_buffer[window_index] = pixel.linear_to_gamma().rgb_value();
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

fn bouncing_spheres() -> (HittableList, Camera) {
    let mut world: HittableList = HittableList::new();

    let checker: Box<dyn Texture> = Box::new(CheckerTexture::from_color(
        0.32,
        &Color::new(Vec3::new(0.2, 0.3, 0.1)),
        &Color::new(Vec3::new(0.9, 0.9, 0.9)),
    ));
    let ground_material = Arc::new(Lambertian::from(checker));
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
                    sphere_material = Arc::new(Lambertian::from(albedo));
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
    let material2 = Arc::new(Lambertian::from(vec3(0.4, 0.2, 0.1)));
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

    let world = HittableList::from(Arc::new(BvhNode::from(world)) as Arc<dyn Hittable>);

    let mut properties = CameraProperties::default();

    properties.aspect_ratio = 16.0 / 9.0;
    properties.image_width = 1000;
    properties.samples_per_pixel = 15;
    properties.max_depth = 50;
    properties.background = vec3(0.7, 0.8, 1.0);
    properties.v_fov = 20.0;
    properties.look_from = vec3(13.0, 2.0, 3.0);
    properties.look_at = vec3(0.0, 0.0, -1.0);
    properties.up = vec3(0.0, 1.0, 0.0);
    properties.defocus_angle = 0.6;
    properties.focus_dist = 10.0;

    let camera = Camera::new(properties);

    (world, camera)
}

fn checkered_spheres() -> (HittableList, Camera) {
    let mut world: HittableList = HittableList::new();

    let checker: Box<dyn Texture> = Box::new(CheckerTexture::from_color(
        0.32,
        &Color::new(Vec3::new(0.2, 0.3, 0.1)),
        &Color::new(Vec3::new(0.9, 0.9, 0.9)),
    ));
    let checker = Arc::new(Lambertian::from(checker));
    world.add(Arc::new(Sphere::new_stationary(
        vec3(0.0, -10.0, 0.0),
        10.0,
        checker.clone(),
    )));
    world.add(Arc::new(Sphere::new_stationary(
        vec3(0.0, 10.0, 0.0),
        10.0,
        checker,
    )));

    let world = HittableList::from(Arc::new(BvhNode::from(world)) as Arc<dyn Hittable>);

    let mut properties = CameraProperties::default();

    properties.aspect_ratio = 16.0 / 9.0;
    properties.image_width = 400;
    properties.samples_per_pixel = 15;
    properties.max_depth = 50;
    properties.background = vec3(0.7, 0.8, 1.0);
    properties.v_fov = 20.0;
    properties.look_from = vec3(13.0, 2.0, 3.0);
    properties.look_at = vec3(0.0, 0.0, 0.0);
    properties.up = vec3(0.0, 1.0, 0.0);
    properties.defocus_angle = 0.6;

    let camera = Camera::new(properties);

    (world, camera)
}

fn earth() -> (HittableList, Camera) {
    let earth_texture: Box<dyn Texture> = Box::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::from(earth_texture));
    let globe: Arc<dyn Hittable> = Arc::new(Sphere::new_stationary(
        vec3(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    ));

    let world = HittableList::from(globe);

    let mut properties = CameraProperties::default();

    properties.aspect_ratio = 16.0 / 9.0;
    properties.image_width = 400;
    properties.samples_per_pixel = 15;
    properties.max_depth = 50;
    properties.background = vec3(0.7, 0.8, 1.0);
    properties.v_fov = 20.0;
    properties.look_from = vec3(0.0, 0.0, 12.0);
    properties.look_at = vec3(0.0, 0.0, 0.0);
    properties.up = vec3(0.0, 1.0, 0.0);
    // properties.defocus_angle = 0.6;

    let camera = Camera::new(properties);

    (world, camera)
}

fn perlin_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let perlin_texture: Box<dyn Texture> = Box::new(NoiseTexture::new(4.0));
    let perlin_surface = Arc::new(Lambertian::from(perlin_texture));
    world.add(Arc::new(Sphere::new_stationary(
        vec3(0.0, -1_000.0, 0.0),
        1_000.0,
        perlin_surface.clone(),
    )));
    world.add(Arc::new(Sphere::new_stationary(
        vec3(0.0, 2.0, 0.0),
        2.0,
        perlin_surface.clone(),
    )));

    let mut properties = CameraProperties::default();

    properties.aspect_ratio = 16.0 / 9.0;
    properties.image_width = 400;
    properties.samples_per_pixel = 15;
    properties.max_depth = 50;
    properties.background = vec3(0.7, 0.8, 1.0);
    properties.v_fov = 20.0;
    properties.look_from = vec3(13.0, 2.0, 3.0);
    properties.look_at = vec3(0.0, 0.0, 0.0);
    properties.up = vec3(0.0, 1.0, 0.0);
    properties.defocus_angle = 0.0;

    let camera = Camera::new(properties);

    (world, camera)
}

fn quads() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let left_red = Arc::new(Lambertian::from(Vec3::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::from(Vec3::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::from(Vec3::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::from(Vec3::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::from(Vec3::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(
        vec3(-3.0, -2.0, 5.0),
        vec3(0.0, 0.0, -4.0),
        vec3(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        vec3(-2.0, -2.0, 0.0),
        vec3(4.0, 0.0, 0.0),
        vec3(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        vec3(3.0, -2.0, 1.0),
        vec3(0.0, 0.0, 4.0),
        vec3(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        vec3(-2.0, 3.0, 1.0),
        vec3(4.0, 0.0, 0.0),
        vec3(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        vec3(-2.0, -3.0, 5.0),
        vec3(4.0, 0.0, 0.0),
        vec3(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let mut properties = CameraProperties::default();

    properties.aspect_ratio = 1.0;
    properties.image_width = 400;
    properties.samples_per_pixel = 15;
    properties.max_depth = 50;
    properties.background = vec3(0.7, 0.8, 1.0);
    properties.v_fov = 80.0;
    properties.look_from = vec3(0.0, 0.0, 9.0);
    properties.look_at = vec3(0.0, 0.0, 0.0);
    properties.up = vec3(0.0, 1.0, 0.0);
    properties.defocus_angle = 0.0;

    let camera = Camera::new(properties);

    (world, camera)
}

fn simple_light() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let perlin_texture: Box<dyn Texture> = Box::new(NoiseTexture::new(4.0));
    let perlin_surface = Arc::new(Lambertian::from(perlin_texture));
    world.add(Arc::new(Sphere::new_stationary(
        vec3(0.0, -1_000.0, 0.0),
        1_000.0,
        perlin_surface.clone(),
    )));
    world.add(Arc::new(Sphere::new_stationary(
        vec3(0.0, 2.0, 0.0),
        2.0,
        perlin_surface.clone(),
    )));

    let diffuse_light = Arc::new(DiffuseLight::from(Vec3::splat(4.0)));
    world.add(Arc::new(Sphere::new_stationary(
        vec3(0.0, 7.0, 0.0),
        2.0,
        diffuse_light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        vec3(3.0, 1.0, -2.0),
        vec3(2.0, 0.0, 0.0),
        vec3(0.0, 2.0, 0.0),
        diffuse_light,
    )));

    let mut properties = CameraProperties::default();

    properties.aspect_ratio = 16.0 / 9.0;
    properties.image_width = 400;
    properties.samples_per_pixel = 15;
    properties.max_depth = 50;
    properties.background = Vec3::splat(0.0);
    properties.v_fov = 20.0;
    properties.look_from = vec3(26.0, 3.0, 6.0);
    properties.look_at = vec3(0.0, 2.0, 0.0);
    properties.up = vec3(0.0, 1.0, 0.0);
    properties.defocus_angle = 0.0;

    let camera = Camera::new(properties);

    (world, camera)
}
