mod buffer;
mod color;
mod hittable;
mod image;
mod interval;
mod camera;
mod utils;

use crate::buffer::DrawBuffer;
use crate::camera::{Camera, CameraProperties};
use crate::hittable::hittable_list::HittableList;
use crate::hittable::sphere::Sphere;
use crate::image::ppm_image::PpmImage;
use crate::image::window_image::WindowImage;
use glam::{vec3, Vec3};

fn main() {
    let mut world: HittableList = HittableList::new();

    world.add(Box::new(Sphere::new(vec3(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(vec3(0.0, -100.5, -1.0), 100.0)));

    let properties = CameraProperties::default().set_aspect_ratio(16.0 / 9.0).set_image_width(400);
    let camera = Camera::new(properties);

    let buffer =  camera.render(&world);

        let image = PpmImage::new("image.ppm");
        image.draw_buffer(&buffer);
        let image = WindowImage::new("raytracing.rs");
        image.draw_buffer(&buffer);
}

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
