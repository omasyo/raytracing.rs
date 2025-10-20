pub mod aabb;
pub mod bvh;
pub mod hittable_list;
pub mod sphere;

use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3;
use std::sync::Arc;
use crate::hittable::aabb::Aabb;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
}

impl HitRecord {
    fn new(
        point: Vec3,
        t: f32,
        ray: &Ray,
        outward_normal: Vec3,
        material: Arc<dyn Material>,
        (u, v): (f32, f32),
    ) -> Self {
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
            u,
            v,
            front_face,
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, ray_interval: Interval) -> Option<HitRecord>;


    fn bounding_box(&self) -> &Aabb {
        &Aabb::EMPTY
    }
}
