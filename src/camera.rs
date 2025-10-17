use crate::buffer::{Buffer, linear_to_gamma};
use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::ScatterResult;
use crate::ray::Ray;
use crate::utils::random_in_unit_disk;
use glam::{Vec3, vec3};
use rand::prelude::SliceRandom;
use rayon::prelude::*;
use std::cmp::max;
use std::sync::mpsc::Sender;

pub struct CameraProperties {
    pub aspect_ratio: f32,
    pub image_width: usize,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub v_fov: f32,
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
    pub defocus_angle: f32,
    pub focus_dist: f32,
}

impl Default for CameraProperties {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 50,
            v_fov: 90.0,
            look_from: vec3(0.0, 0.0, 0.0),
            look_at: vec3(0.0, 0.0, -1.0),
            up: vec3(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
        }
    }
}

pub struct Camera {
    pub image_width: usize,
    pub image_height: usize,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u32,
    max_depth: u32,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_angle: f32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(properties: CameraProperties) -> Self {
        let aspect_ratio = properties.aspect_ratio;
        let image_width = properties.image_width;
        let defocus_angle = properties.defocus_angle;

        let image_height = (image_width as f32 / aspect_ratio) as usize;
        let image_height = max(image_height, 1);

        let center = properties.look_from;

        let theta = properties.v_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * properties.focus_dist;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        let w = (properties.look_from - properties.look_at).normalize();
        let u = properties.up.cross(w).normalize();
        let v = w.cross(u).normalize();

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / (image_width as f32);
        let pixel_delta_v = viewport_v / (image_height as f32);

        let viewport_upper_left =
            center - (properties.focus_dist * w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius =
            properties.focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel: properties.samples_per_pixel,
            max_depth: properties.max_depth,
            u,
            v,
            w,
            defocus_angle: properties.defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &HittableList, tx: Sender<Buffer>) {
        let mut buffer = Buffer::new(self.image_width, self.image_height);
        let mut loop_count = 1.0;

        loop {
            buffer
                .data
                .par_iter_mut()
                .enumerate()
                .for_each(|(index, pixel)| {
                    let j = index / self.image_width;
                    let i = index % self.image_width;
                    // let mut pixel_color = vec3(0.0, 0.0, 0.0);
                    // for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i as f32, j as f32);
                    let pixel_color = ray_color(&ray, self.max_depth, world);
                    // }
                    // pixel_color /= self.samples_per_pixel as f32;
                    let new_color =/* Color::from_vec3(*/Vec3 {
                        x: linear_to_gamma(pixel_color.x),
                        y: linear_to_gamma(pixel_color.y),
                        z: linear_to_gamma(pixel_color.z),
                    }/*)*/;
                    let old_color = pixel.to_vec3();
                    let color = (old_color * (loop_count - 1.0) / loop_count)
                        + (new_color * (1.0 / loop_count));
                    *pixel = Color::from_vec3(color);
                });
            println!("Loop {loop_count}");
            loop_count = loop_count + 1.0;
            tx.send(buffer.clone()).unwrap();
        }

        // (0..(self.image_width * self.image_height))
        // shuffled_range(0, self.image_width * self.image_height)
        //     .into_par_iter()
        //     .for_each(|index| {
        //         let j = index / self.image_width;
        //         let i = index % self.image_width;
        //         let mut pixel_color = vec3(0.0, 0.0, 0.0);
        //         for _ in 0..self.samples_per_pixel {
        //             let ray = self.get_ray(i as f32, j as f32);
        //             pixel_color += ray_color(&ray, self.max_depth, world);
        //         }
        //         pixel_color /= self.samples_per_pixel as f32;
        //         let pixel = Color::from_vec3(Vec3 {
        //             x: linear_to_gamma(pixel_color.x),
        //             y: linear_to_gamma(pixel_color.y),
        //             z: linear_to_gamma(pixel_color.z),
        //         });
        //         unsafe {
        //             buffer.write().unwrap().write_at(index, pixel);
        //         }
        //         tx.send(buffer.read().unwrap().clone()).expect("TODO: panic message");
        //     });
    }

    fn get_ray(&self, i: f32, j: f32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i + offset.x) * self.pixel_delta_u)
            + ((j + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        }; // todo inline
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
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

fn shuffled_range(start: usize, end: usize) -> Vec<usize> {
    assert!(start < end);
    let mut v: Vec<usize> = (start..end).collect();
    v.shuffle(&mut rand::rng());
    v
}
