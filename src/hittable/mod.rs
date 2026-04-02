pub mod aabb;
pub mod bvh;
pub mod constant_medium;
pub mod hittable_list;
pub mod quad;
pub mod sphere;
pub mod transform;

use crate::hittable::aabb::Aabb;
use crate::interval::Interval;
use crate::material::MaterialKind;
use crate::ray::Ray;
use glam::Vec3;

pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a MaterialKind,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    fn new(
        point: Vec3,
        t: f32,
        ray: &Ray,
        outward_normal: Vec3,
        material: &'a MaterialKind,
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
    fn hit(&self, r: &Ray, ray_interval: Interval) -> Option<HitRecord<'_>>;
    fn bounding_box(&self) -> &Aabb;
}
