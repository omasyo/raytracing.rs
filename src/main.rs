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
use crate::hittable::Hittable;
use crate::hittable::bvh::BvhNode;
use crate::hittable::constant_medium::ConstantMedium;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::quad::{Quad, cuboid};
use crate::hittable::sphere::Sphere;
use crate::hittable::transform::{RotateY, Translate};
use crate::image::ppm_image::PpmImage;
use crate::material::MaterialKind;
use crate::material::texture::TextureKind;
use crate::utils::{random_vector, random_vector_range};
use crate::window::{SoftbufferWindow, WindowProperties};
use glam::{Vec3, vec3};
use std::thread;
use winit::event::WindowEvent;

fn main() {
    let (world, camera) = match 9 {
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

    let (tx, rx) = std::sync::mpsc::channel::<Buffer>();
    thread::spawn(move || {
        camera.render(&world, tx);
    });

    window
        .run(move |window, event| match event {
            WindowEvent::RedrawRequested => {
                let (width, _height) = window.inner_size();
                while let Ok(b) = rx.try_recv() {
                    buffer = b;
                }
                let mut window_buffer = window.buffer_mut();
                for (index, pixel) in buffer.data.iter().enumerate() {
                    let x = index % buffer.width;
                    let y = index / buffer.width;
                    let window_index = (y * width) + x;
                    window_buffer[window_index] = pixel.linear_to_gamma().rgb_value();
                }
                window_buffer.present().unwrap();
            }
            WindowEvent::CloseRequested => {
                let image = PpmImage::new("image.ppm");
                image.draw_buffer(&buffer);
            }
            _ => {}
        })
        .expect("window can't run :(");
}

fn bouncing_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let checker = TextureKind::checker(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));
    world.add(Box::new(Sphere::new_stationary(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        MaterialKind::lambertian_textured(checker),
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
                if choose_mat < 0.8 {
                    let albedo = random_vector() * random_vector();
                    let center2 = center + vec3(0.0, rand::random_range(0.0..=0.5), 0.0);
                    world.add(Box::new(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        MaterialKind::lambertian(albedo),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = random_vector_range(0.5, 1.0);
                    let fuzz = rand::random_range(0.0..=0.5);
                    world.add(Box::new(Sphere::new_stationary(
                        center,
                        0.2,
                        MaterialKind::metal(albedo, fuzz),
                    )));
                } else {
                    world.add(Box::new(Sphere::new_stationary(
                        center,
                        0.2,
                        MaterialKind::dielectric(1.5),
                    )));
                }
            }
        }
    }

    world.add(Box::new(Sphere::new_stationary(
        vec3(0.0, 1.0, 0.0),
        1.0,
        MaterialKind::dielectric(1.5),
    )));
    world.add(Box::new(Sphere::new_stationary(
        vec3(-4.0, 1.0, 0.0),
        1.0,
        MaterialKind::lambertian(vec3(0.4, 0.2, 0.1)),
    )));
    world.add(Box::new(Sphere::new_stationary(
        vec3(4.0, 1.0, 0.0),
        1.0,
        MaterialKind::metal(vec3(0.7, 0.6, 0.5), 0.0),
    )));

    let world = HittableList::from(Box::new(BvhNode::from(world)) as Box<dyn Hittable>);

    let camera = Camera::new(CameraProperties {
        aspect_ratio: 16.0 / 9.0,
        image_width: 1000,
        samples_per_pixel: Some(15),
        max_depth: 50,
        background: vec3(0.7, 0.8, 1.0),
        v_fov: 20.0,
        look_from: vec3(13.0, 2.0, 3.0),
        look_at: vec3(0.0, 0.0, -1.0),
        defocus_angle: 0.6,
        focus_dist: 10.0,
        ..Default::default()
    });

    (world, camera)
}

fn checkered_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let checker = MaterialKind::lambertian_textured(TextureKind::checker(
        0.32,
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    world.add(Box::new(Sphere::new_stationary(
        vec3(0.0, -10.0, 0.0),
        10.0,
        checker.clone(),
    )));
    world.add(Box::new(Sphere::new_stationary(
        vec3(0.0, 10.0, 0.0),
        10.0,
        checker,
    )));

    let world = HittableList::from(Box::new(BvhNode::from(world)) as Box<dyn Hittable>);

    let camera = Camera::new(CameraProperties {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: Some(15),
        max_depth: 50,
        background: vec3(0.7, 0.8, 1.0),
        v_fov: 20.0,
        look_from: vec3(13.0, 2.0, 3.0),
        look_at: vec3(0.0, 0.0, 0.0),
        defocus_angle: 0.6,
        ..Default::default()
    });

    (world, camera)
}

fn earth() -> (HittableList, Camera) {
    let globe: Box<dyn Hittable> = Box::new(Sphere::new_stationary(
        vec3(0.0, 0.0, 0.0),
        2.0,
        MaterialKind::lambertian_textured(TextureKind::image("earthmap.jpg")),
    ));

    let world = HittableList::from(globe);

    let camera = Camera::new(CameraProperties {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: Some(15),
        max_depth: 50,
        background: vec3(0.7, 0.8, 1.0),
        v_fov: 20.0,
        look_from: vec3(0.0, 0.0, 12.0),
        look_at: vec3(0.0, 0.0, 0.0),
        ..Default::default()
    });

    (world, camera)
}

fn perlin_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let perlin = MaterialKind::lambertian_textured(TextureKind::noise(4.0));
    world.add(Box::new(Sphere::new_stationary(
        vec3(0.0, -1_000.0, 0.0),
        1_000.0,
        perlin.clone(),
    )));
    world.add(Box::new(Sphere::new_stationary(
        vec3(0.0, 2.0, 0.0),
        2.0,
        perlin,
    )));

    let camera = Camera::new(CameraProperties {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: Some(15),
        max_depth: 50,
        background: vec3(0.7, 0.8, 1.0),
        v_fov: 20.0,
        look_from: vec3(13.0, 2.0, 3.0),
        look_at: vec3(0.0, 0.0, 0.0),
        ..Default::default()
    });

    (world, camera)
}

fn quads() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    // Left red
    world.add(Box::new(Quad::new(
        vec3(-3.0, -2.0, 5.0),
        vec3(0.0, 0.0, -4.0),
        vec3(0.0, 4.0, 0.0),
        MaterialKind::lambertian(Vec3::new(1.0, 0.2, 0.2)),
    )));

    // Back green
    world.add(Box::new(Quad::new(
        vec3(-2.0, -2.0, 0.0),
        vec3(4.0, 0.0, 0.0),
        vec3(0.0, 4.0, 0.0),
        MaterialKind::lambertian(Vec3::new(0.2, 1.0, 0.2)),
    )));

    // Right blue
    world.add(Box::new(Quad::new(
        vec3(3.0, -2.0, 1.0),
        vec3(0.0, 0.0, 4.0),
        vec3(0.0, 4.0, 0.0),
        MaterialKind::lambertian(Vec3::new(0.2, 0.2, 1.0)),
    )));

    // Upper orange
    world.add(Box::new(Quad::new(
        vec3(-2.0, 3.0, 1.0),
        vec3(4.0, 0.0, 0.0),
        vec3(0.0, 0.0, 4.0),
        MaterialKind::lambertian(Vec3::new(1.0, 0.5, 0.0)),
    )));

    // Lower teal
    world.add(Box::new(Quad::new(
        vec3(-2.0, -3.0, 5.0),
        vec3(4.0, 0.0, 0.0),
        vec3(0.0, 0.0, -4.0),
        MaterialKind::lambertian(Vec3::new(0.2, 0.8, 0.8)),
    )));

    let camera = Camera::new(CameraProperties {
        aspect_ratio: 1.0,
        image_width: 400,
        samples_per_pixel: Some(15),
        max_depth: 50,
        background: vec3(0.7, 0.8, 1.0),
        v_fov: 80.0,
        look_from: vec3(0.0, 0.0, 9.0),
        look_at: vec3(0.0, 0.0, 0.0),
        ..Default::default()
    });

    (world, camera)
}

fn simple_light() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let perlin = MaterialKind::lambertian_textured(TextureKind::noise(4.0));
    world.add(Box::new(Sphere::new_stationary(
        vec3(0.0, -1_000.0, 0.0),
        1_000.0,
        perlin.clone(),
    )));
    world.add(Box::new(Sphere::new_stationary(
        vec3(0.0, 2.0, 0.0),
        2.0,
        perlin,
    )));

    let light = MaterialKind::diffuse_light(Vec3::splat(4.0));
    world.add(Box::new(Sphere::new_stationary(
        vec3(0.0, 7.0, 0.0),
        2.0,
        light.clone(),
    )));
    world.add(Box::new(Quad::new(
        vec3(3.0, 1.0, -2.0),
        vec3(2.0, 0.0, 0.0),
        vec3(0.0, 2.0, 0.0),
        light,
    )));

    let camera = Camera::new(CameraProperties {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: Some(15),
        max_depth: 50,
        background: Vec3::ZERO,
        v_fov: 20.0,
        look_from: vec3(26.0, 3.0, 6.0),
        look_at: vec3(0.0, 2.0, 0.0),
        ..Default::default()
    });

    (world, camera)
}

fn cornell_box() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let red = MaterialKind::lambertian(Vec3::new(0.65, 0.05, 0.05));
    let white = MaterialKind::lambertian(Vec3::new(0.73, 0.73, 0.73));
    let green = MaterialKind::lambertian(Vec3::new(0.12, 0.45, 0.15));
    let light = MaterialKind::diffuse_light(Vec3::splat(15.0));

    world.add(Box::new(Quad::new(
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Box::new(Quad::new(
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Box::new(Quad::new(
        vec3(343.0, 554.0, 332.0),
        vec3(-130.0, 0.0, 0.0),
        vec3(0.0, 0.0, -105.0),
        light,
    )));
    world.add(Box::new(Quad::new(
        vec3(0.0, 0.0, 0.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Box::new(Quad::new(
        vec3(555.0, 555.0, 555.0),
        vec3(-555.0, 0.0, 0.0),
        vec3(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Box::new(Quad::new(
        vec3(0.0, 0.0, 555.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let mut box1: Box<dyn Hittable> = Box::new(cuboid(
        vec3(0.0, 0.0, 0.0),
        vec3(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Box::new(RotateY::new(box1, 15.0));
    box1 = Box::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let mut box2: Box<dyn Hittable> = Box::new(cuboid(
        vec3(0.0, 0.0, 0.0),
        vec3(165.0, 165.0, 165.0),
        white,
    ));
    box2 = Box::new(RotateY::new(box2, -18.0));
    box2 = Box::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    let camera = Camera::new(CameraProperties {
        aspect_ratio: 1.0,
        image_width: 600,
        samples_per_pixel: Some(64),
        max_depth: 50,
        background: Vec3::ZERO,
        v_fov: 40.0,
        look_from: vec3(278.0, 278.0, -800.0),
        look_at: vec3(278.0, 278.0, 0.0),
        ..Default::default()
    });

    (world, camera)
}

fn cornell_smoke() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let red = MaterialKind::lambertian(Vec3::new(0.65, 0.05, 0.05));
    let white = MaterialKind::lambertian(Vec3::new(0.73, 0.73, 0.73));
    let green = MaterialKind::lambertian(Vec3::new(0.12, 0.45, 0.15));
    let light = MaterialKind::diffuse_light(Vec3::splat(7.0));

    world.add(Box::new(Quad::new(
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Box::new(Quad::new(
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Box::new(Quad::new(
        vec3(113.0, 554.0, 127.0),
        vec3(330.0, 0.0, 0.0),
        vec3(0.0, 0.0, 305.0),
        light,
    )));
    world.add(Box::new(Quad::new(
        vec3(0.0, 555.0, 0.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Box::new(Quad::new(
        vec3(0.0, 0.0, 0.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Box::new(Quad::new(
        vec3(0.0, 0.0, 555.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let mut box1: Box<dyn Hittable> = Box::new(cuboid(
        vec3(0.0, 0.0, 0.0),
        vec3(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Box::new(RotateY::new(box1, 15.0));
    box1 = Box::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let mut box2: Box<dyn Hittable> = Box::new(cuboid(
        vec3(0.0, 0.0, 0.0),
        vec3(165.0, 165.0, 165.0),
        white,
    ));
    box2 = Box::new(RotateY::new(box2, -18.0));
    box2 = Box::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    world.add(Box::new(ConstantMedium::from_color(box1, 0.01, Vec3::ZERO)));
    world.add(Box::new(ConstantMedium::from_color(box2, 0.01, Vec3::ONE)));

    let camera = Camera::new(CameraProperties {
        aspect_ratio: 1.0,
        image_width: 600,
        samples_per_pixel: Some(15),
        max_depth: 50,
        background: Vec3::ZERO,
        v_fov: 40.0,
        look_from: vec3(278.0, 278.0, -800.0),
        look_at: vec3(278.0, 278.0, 0.0),
        ..Default::default()
    });

    (world, camera)
}

fn final_scene(
    image_width: usize,
    samples_per_pixel: Option<u32>,
    max_depth: u32,
) -> (HittableList, Camera) {
    let mut boxes1 = HittableList::new();
    let ground = MaterialKind::lambertian(Vec3::new(0.48, 0.83, 0.53));

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

            boxes1.add(Box::new(cuboid(
                vec3(x0, y0, z0),
                vec3(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut world = HittableList::new();

    world.add(Box::new(BvhNode::from(boxes1)));

    world.add(Box::new(Quad::new(
        vec3(123.0, 554.0, 147.0),
        vec3(300.0, 0.0, 0.0),
        vec3(0.0, 0.0, 265.0),
        MaterialKind::diffuse_light(Vec3::splat(7.0)),
    )));

    let center1 = vec3(400.0, 400.0, 200.0);
    let center2 = center1 + vec3(30.0, 0.0, 0.0);
    world.add(Box::new(Sphere::new_moving(
        center1,
        center2,
        50.0,
        MaterialKind::lambertian(Vec3::new(0.7, 0.3, 0.1)),
    )));

    world.add(Box::new(Sphere::new_stationary(
        vec3(260.0, 150.0, 45.0),
        50.0,
        MaterialKind::dielectric(1.5),
    )));
    world.add(Box::new(Sphere::new_stationary(
        vec3(0.0, 150.0, 45.0),
        50.0,
        MaterialKind::metal(vec3(0.8, 0.8, 0.9), 1.0),
    )));

    let boundary_mat = MaterialKind::dielectric(1.5);
    world.add(Box::new(Sphere::new_stationary(
        vec3(360.0, 150.0, 45.0),
        70.0,
        boundary_mat.clone(),
    )));
    world.add(Box::new(ConstantMedium::from_color(
        Box::new(Sphere::new_stationary(
            vec3(360.0, 150.0, 45.0),
            70.0,
            boundary_mat,
        )),
        0.2,
        vec3(0.2, 0.4, 0.9),
    )));

    world.add(Box::new(ConstantMedium::from_color(
        Box::new(Sphere::new_stationary(
            vec3(0.0, 0.0, 0.0),
            5000.0,
            MaterialKind::dielectric(1.5),
        )),
        0.0001,
        Vec3::ONE,
    )));

    world.add(Box::new(Sphere::new_stationary(
        vec3(400.0, 200.0, 400.0),
        100.0,
        MaterialKind::lambertian_textured(TextureKind::image("earthmap.jpg")),
    )));
    world.add(Box::new(Sphere::new_stationary(
        vec3(200.0, 280.0, 300.0),
        80.0,
        MaterialKind::lambertian_textured(TextureKind::noise(0.2)),
    )));

    let mut boxes2 = HittableList::new();
    let white = MaterialKind::lambertian(Vec3::new(0.73, 0.73, 0.73));
    for _ in 0..1_000 {
        boxes2.add(Box::new(Sphere::new_stationary(
            random_vector_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    world.add(Box::new(Translate::new(
        Box::new(RotateY::new(Box::new(BvhNode::from(boxes2)), 15.0)),
        vec3(-100.0, 270.0, 395.0),
    )));

    let camera = Camera::new(CameraProperties {
        aspect_ratio: 1.0,
        image_width,
        samples_per_pixel,
        max_depth,
        background: Vec3::ZERO,
        v_fov: 40.0,
        look_from: vec3(478.0, 278.0, -600.0),
        look_at: vec3(278.0, 278.0, 0.0),
        ..Default::default()
    });

    (world, camera)
}
