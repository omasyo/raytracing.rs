mod buffer;
mod camera;
mod color;
mod hittable;
mod image;
mod interval;
mod utils;
mod ray;
mod material;

use std::rc::Rc;
use crate::buffer::DrawBuffer;
use crate::camera::{Camera, CameraProperties};
use crate::hittable::hittable_list::HittableList;
use crate::hittable::sphere::Sphere;
use crate::image::ppm_image::PpmImage;
use crate::image::window_image::WindowImage;
use glam::{Vec3, vec3};
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;

fn main() {
    let mut world: HittableList = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(vec3(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(vec3(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(vec3(0.8, 0.8, 0.9)));
    let material_right = Rc::new(Metal::new(vec3(0.8, 0.6, 0.2)));

    world.add(Box::new(Sphere::new(vec3(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(vec3(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Box::new(Sphere::new(vec3(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(vec3(1.0, 0.0, -1.0), 0.5, material_right)));

    let properties = CameraProperties::default()
        .set_aspect_ratio(16.0 / 9.0)
        .set_image_width(400)
        .set_samples_per_pixel(100)
        .set_max_depth(50);

    let camera = Camera::new(properties);

    let buffer = camera.render(&world);

    let image = PpmImage::new("image.ppm");
    image.draw_buffer(&buffer);
    // let image = WindowImage::new("raytracing.rs");
    // image.draw_buffer(&buffer);
}