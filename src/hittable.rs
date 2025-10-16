pub mod hittable_list;
pub mod sphere;

use crate::Ray;
use crate::interval::Interval;
use glam::Vec3;

#[derive(Default)]
pub struct HitRecord {
    point: Vec3,
    pub normal: Vec3,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    fn new(point: Vec3, t: f32, ray: &Ray, outward_normal: Vec3) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_interval: Interval) -> Option<HitRecord>;
}
