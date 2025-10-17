use crate::buffer::{Buffer, linear_to_gamma};
use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::ScatterResult;
use crate::ray::Ray;
use crate::utils::random_unit_vector;
use glam::{Vec3, vec3};
use rayon::prelude::*;
use std::cmp::max;
use std::sync::mpsc::Sender;

pub struct CameraProperties {
    aspect_ratio: f32,
    image_width: usize,
    samples_per_pixel: u32,
    max_depth: u32,
}

impl Default for CameraProperties {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 50,
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

    pub fn set_max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
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
    max_depth: u32,
}

impl Camera {
    pub fn new(properties: CameraProperties) -> Self {
        let aspect_ratio = properties.aspect_ratio;
        let image_width = properties.image_width;
        let samples_per_pixel = properties.samples_per_pixel;
        let max_depth = properties.max_depth;

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
            max_depth,
        }
    }

    pub fn render(&self, world: &HittableList, tx: Sender<(usize, Color)>)  {
        Buffer::new(self.image_width, self.image_height);
        (0..(self.image_width*self.image_height)).into_par_iter().for_each(|index| {
            let j = index / self.image_width;
            let i = index % self.image_width;
            let mut pixel_color = vec3(0.0, 0.0, 0.0);
            for _ in 0..self.samples_per_pixel {
                let ray = self.get_ray(i as f32, j as f32);
                pixel_color += ray_color(&ray, self.max_depth, world);
            }
            pixel_color /= self.samples_per_pixel as f32;
            let pixel = Color::from_vec3(Vec3 {
                x: linear_to_gamma(pixel_color.x),
                y: linear_to_gamma(pixel_color.y),
                z: linear_to_gamma(pixel_color.z),
            });
            tx.send((index, pixel)).expect("TODO: panic message");
        });
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
        rand::random_range(0.0..=1.0) - 0.5,
        rand::random_range(0.0..=1.0) - 0.5,
        0.0,
    )
}

fn ray_color(ray: &Ray, depth: u32, world: &dyn Hittable) -> Vec3 {
    if depth <= 0 {
        return vec3(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(ray, Interval::new(0.0001, f32::INFINITY)) {
        if let Some(ScatterResult {
            attenuation,
            scattered,
        }) = rec.material.scatter(ray, &rec)
        {
            return attenuation * ray_color(&scattered, depth - 1, world);
        }
        return vec3(0.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * vec3(1.0, 1.0, 1.0) + a * vec3(0.5, 0.7, 1.0)
}
