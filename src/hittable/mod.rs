pub mod hittable_list;
pub mod sphere;

use std::rc::Rc;
use crate::interval::Interval;
use glam::Vec3;
use crate::material::Material;
use crate::ray::Ray;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f32,
    front_face: bool,
}

impl HitRecord {
    fn new(point: Vec3, t: f32, ray: &Ray, outward_normal: Vec3, material: Rc<dyn Material>) -> Self {
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
