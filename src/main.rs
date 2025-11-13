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
use crate::hittable::constant_medium::ConstantMedium;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::quad::{Quad, cuboid};
use crate::hittable::sphere::Sphere;
use crate::hittable::transform::{RotateY, Translate};
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
use sobol_burley::sample;
use winit::event::WindowEvent;

fn main() {
    let (world, camera) = match 7 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(400, Some(10_000), 40),
        _ => final_scene(400, Some(250), 4),
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
    properties.samples_per_pixel = Some(15);
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
    properties.samples_per_pixel = Some(15);
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
    properties.samples_per_pixel = Some(15);
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
    properties.samples_per_pixel = Some(15);
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
    properties.samples_per_pixel = Some(15);
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
    properties.samples_per_pixel = Some(15);
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

fn cornell_box() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::from(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from(Vec3::splat(15.0)));

    world.add(Arc::new(Quad::new(
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        vec3(343.0, 554.0, 332.0),
        vec3(-130.0, 0.0, 0.0),
        vec3(0.0, 0.0, -105.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        vec3(0.0, 0.0, 0.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        vec3(555.0, 555.0, 555.0),
        vec3(-555.0, 0.0, 0.0),
        vec3(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        vec3(0.0, 0.0, 555.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let mut box1: Arc<dyn Hittable> = Arc::new(cuboid(
        vec3(0.0, 0.0, 0.0),
        vec3(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let mut box2: Arc<dyn Hittable> = Arc::new(cuboid(
        vec3(0.0, 0.0, 0.0),
        vec3(165.0, 165.0, 165.0),
        white.clone(),
    ));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    // let world = HittableList::from(Arc::new(BvhNode::from(world)) as Arc<dyn Hittable>);

    let mut properties = CameraProperties::default();

    properties.aspect_ratio = 1.0;
    properties.image_width = 600;
    properties.samples_per_pixel = Some(64);
    properties.max_depth = 50;
    properties.background = Vec3::splat(0.0);
    properties.v_fov = 40.0;
    properties.look_from = vec3(278.0, 278.0, -800.0);
    properties.look_at = vec3(278.0, 278.0, 0.0);
    properties.up = vec3(0.0, 1.0, 0.0);
    properties.defocus_angle = 0.0;

    let camera = Camera::new(properties);

    (world, camera)
}

fn cornell_smoke() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::from(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from(Vec3::splat(7.0)));

    world.add(Arc::new(Quad::new(
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        vec3(113.0, 554.0, 127.0),
        vec3(330.0, 0.0, 0.0),
        vec3(0.0, 0.0, 305.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        vec3(0.0, 555.0, 0.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        vec3(0.0, 0.0, 0.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        vec3(0.0, 0.0, 555.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let mut box1: Arc<dyn Hittable> = Arc::new(cuboid(
        vec3(0.0, 0.0, 0.0),
        vec3(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let mut box2: Arc<dyn Hittable> = Arc::new(cuboid(
        vec3(0.0, 0.0, 0.0),
        vec3(165.0, 165.0, 165.0),
        white.clone(),
    ));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    world.add(Arc::new(ConstantMedium::from_color(
        box1,
        0.01,
        Vec3::splat(0.0),
    )));
    world.add(Arc::new(ConstantMedium::from_color(
        box2,
        0.01,
        Vec3::splat(1.0),
    )));

    // let world = HittableList::from(Arc::new(BvhNode::from(world)) as Arc<dyn Hittable>);

    let mut properties = CameraProperties::default();

    properties.aspect_ratio = 1.0;
    properties.image_width = 600;
    properties.samples_per_pixel = Some(15);
    properties.max_depth = 50;
    properties.background = Vec3::splat(0.0);
    properties.v_fov = 40.0;
    properties.look_from = vec3(278.0, 278.0, -800.0);
    properties.look_at = vec3(278.0, 278.0, 0.0);
    properties.up = vec3(0.0, 1.0, 0.0);
    properties.defocus_angle = 0.0;

    let camera = Camera::new(properties);

    (world, camera)
}

fn final_scene(
    image_width: usize,
    samples_per_pixel: Option<u32>,
    max_depth: u32,
) -> (HittableList, Camera) {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::from(Vec3::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rand::random_range(0.0..101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(cuboid(
                vec3(x0, y0, z0),
                vec3(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut world = HittableList::new();

    world.add(Arc::new(BvhNode::from(boxes1)));

    let light = Arc::new(DiffuseLight::from(Vec3::splat(7.0)));
    world.add(Arc::new(Quad::new(
        vec3(123.0, 554.0, 147.0),
        vec3(300.0, 0.0, 0.0),
        vec3(0.0, 0.0, 265.0),
        light,
    )));

    let center1 = vec3(400.0, 400.0, 200.0);
    let center2 = center1 + vec3(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::from(Vec3::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_moving(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    world.add(Arc::new(Sphere::new_stationary(
        vec3(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new_stationary(
        vec3(0.0, 150.0, 45.0),
        50.0,
        Arc::new(Metal::new(vec3(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::new_stationary(
        vec3(360.0, 150.0, 45.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::from_color(
        boundary,
        0.2,
        vec3(0.2, 0.4, 0.9),
    )));

    let boundary = Arc::new(Sphere::new_stationary(
        vec3(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::from_color(
        boundary,
        0.0001,
        Vec3::splat(1.0),
    )));

    let e_mat = Arc::new(Lambertian::from(
        Box::new(ImageTexture::new("earthmap.jpg")) as Box<dyn Texture>,
    ));
    world.add(Arc::new(Sphere::new_stationary(
        vec3(400.0, 200.0, 400.0),
        100.0,
        e_mat,
    )));
    let per_mat = Arc::new(Lambertian::from(
        Box::new(NoiseTexture::new(0.2)) as Box<dyn Texture>
    ));
    world.add(Arc::new(Sphere::new_stationary(
        vec3(200.0, 280.0, 300.0),
        80.0,
        per_mat,
    )));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::from(Vec3::new(0.73, 0.73, 0.73)));
    let ns = 1_000;
    for _i in 0..ns {
        boxes2.add(Arc::new(Sphere::new_stationary(
            random_vector_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    world.add(Arc::new(Translate::new(Arc::new(RotateY::new(Arc::new(BvhNode::from(boxes2)), 15.0)), vec3(-100.0, 270.0, 395.0))));

    // let world = HittableList::from(Arc::new(BvhNode::from(world)) as Arc<dyn Hittable>);

    let mut properties = CameraProperties::default();

    properties.aspect_ratio = 1.0;
    properties.image_width = image_width;
    properties.samples_per_pixel = samples_per_pixel;
    properties.max_depth = max_depth;
    properties.background = Vec3::splat(0.0);
    properties.v_fov = 40.0;
    properties.look_from = vec3(478.0, 278.0, -600.0);
    properties.look_at = vec3(278.0, 278.0, 0.0);
    properties.up = vec3(0.0, 1.0, 0.0);
    properties.defocus_angle = 0.0;

    let camera = Camera::new(properties);

    (world, camera)
}
