mod buffer;
mod color;
mod hittable;
mod image;
mod interval;
mod camera;

use crate::buffer::{Buffer, DrawBuffer};
use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::sphere::Sphere;
use crate::image::ppm_image::PpmImage;
use crate::image::window_image::WindowImage;
use crate::interval::Interval;
use glam::{Vec3, vec3};
use crate::camera::{Camera, CameraProperties};

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

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    if let Some(rec) = world.hit(ray, Interval::new(0.0, f32::INFINITY)) {
        let color = 0.5 * (rec.normal + vec3(1.0, 1.0, 1.0));
        return Color::from_vec3(color);
    }

    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    let color = (1.0 - a) * vec3(1.0, 1.0, 1.0) + a * vec3(0.5, 0.7, 1.0);
    Color::from_vec3(color)
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
