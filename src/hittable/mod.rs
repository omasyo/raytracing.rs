pub mod hittable_list;
pub mod sphere;

use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3;
use std::sync::Arc;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material + Sync>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    fn new(point: Vec3, t: f32, ray: &Ray, outward_normal: Vec3, material: Arc<dyn Material + Sync>) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
            material,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_interval: Interval) -> Option<HitRecord>;
}
