use crate::buffer::Buffer;
use crate::color::Color;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::{ray_color, Ray};
use glam::{vec3, Vec3};
use std::cmp::max;

pub struct CameraProperties {
    aspect_ratio: f32,
    image_width: usize,
}

impl Default for CameraProperties {
    fn default() -> Self {
        Self { aspect_ratio: 1.0, image_width: 100 }
    }
}

impl CameraProperties {
    pub fn set_aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn set_image_width(mut self, image_width: usize) -> Self {
        self.image_width = image_width;
        self
    }
}

pub struct Camera {
    aspect_ratio: f32,
    image_width: usize,
    image_height: usize,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(properties: CameraProperties) -> Self {
        let aspect_ratio = properties.aspect_ratio;
        let image_width = properties.image_width;

        let image_height = (image_width as f32 / aspect_ratio) as usize;
        let image_height = max(image_height, 1);

        let center = vec3(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);


        let viewport_u = vec3(viewport_width, 0.0, 0.0);
        let viewport_v = vec3(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / (image_width as f32);
        let pixel_delta_v = viewport_v / (image_height as f32);

        let viewport_upper_left =
            center - vec3(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self { aspect_ratio, image_width, image_height, center, pixel00_loc, pixel_delta_u, pixel_delta_v }
    }

    pub fn render(&self, world: &HittableList) -> Buffer {
        let mut buffer = Buffer::new(self.image_width, self.image_height);

        for y in 0..self.image_height {
            eprintln!("Scan lines remaining: {}", self.image_height - y);
            for x in 0..self.image_width {
                let i = x as f32;
                let j = y as f32;
                let pixel_center = self.pixel00_loc + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let pixel_color = ray_color(&ray, world);

                buffer.push(pixel_color);
            }
        }

        buffer
    }

    fn ray_color(&self, ray: &Ray, world: &dyn Hittable) -> Color {
        if let Some(rec) = world.hit(ray, Interval::new(0.0, f32::INFINITY)) {
            let color = 0.5 * (rec.normal + vec3(1.0, 1.0, 1.0));
            return Color::from_vec3(color);
        }

        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        let color = (1.0 - a) * vec3(1.0, 1.0, 1.0) + a * vec3(0.5, 0.7, 1.0);
        Color::from_vec3(color)
    }
}
