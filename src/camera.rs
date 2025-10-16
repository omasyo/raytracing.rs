use crate::Ray;
use crate::buffer::Buffer;
use crate::hittable::Hittable;
use crate::hittable::hittable_list::HittableList;
use crate::interval::Interval;
use glam::{Vec3, vec3};
use std::cmp::max;

pub struct CameraProperties {
    aspect_ratio: f32,
    image_width: usize,
    samples_per_pixel: u32,
}

impl Default for CameraProperties {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
        }
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

    pub fn set_samples_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }
}

pub struct Camera {
    image_width: usize,
    image_height: usize,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u32,
}

impl Camera {
    pub fn new(properties: CameraProperties) -> Self {
        let aspect_ratio = properties.aspect_ratio;
        let image_width = properties.image_width;
        let samples_per_pixel = properties.samples_per_pixel;

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

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
        }
    }

    pub fn render(&self, world: &HittableList) -> Buffer {
        let mut buffer = Buffer::new(self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprintln!("Scan lines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = vec3(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i as f32, j as f32);
                    pixel_color += ray_color(&ray, world);
                }
                pixel_color /= self.samples_per_pixel as f32;
                buffer.write(pixel_color);
            }
        }

        buffer
    }

    fn get_ray(&self, i: f32, j: f32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i + offset.x) * self.pixel_delta_u)
            + ((j + offset.y) * self.pixel_delta_v);

        let ray_origin = self.center; // todo inline
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(
        rand::random_range(0.0..1.0) - 0.5,
        rand::random_range(0.0..1.0) - 0.5,
        0.0,
    )
}

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Vec3 {
    if let Some(rec) = world.hit(ray, Interval::new(0.0, f32::INFINITY)) {
       return 0.5 * (rec.normal + vec3(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * vec3(1.0, 1.0, 1.0) + a * vec3(0.5, 0.7, 1.0)
}
