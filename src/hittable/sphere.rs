use super::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3;
use std::sync::Arc;

pub struct Sphere {
    center: Ray,
    radius: f32,
    material: Arc<dyn Material + Sync>,
}

impl Sphere {
    pub fn new_stationary(center: Vec3, radius: f32, material: Arc<dyn Material + Sync>) -> Sphere {
        assert!(radius > 0.0);
        Sphere {
            center: Ray::new(center, Vec3::ZERO, 0.0),
            radius,
            material,
        }
    }

    pub fn new_moving(center1: Vec3, center2: Vec3, radius: f32, material: Arc<dyn Material + Sync>) -> Sphere {
        assert!(radius > 0.0);
        Sphere {
            center: Ray::new(center1, center2-center1, 0.0),
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_interval: Interval) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time);
        let oc = current_center - ray.origin;

        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        let mut root = (h - sqrt_d) / a;
        if !ray_interval.surrounds(root) {
            root = (h + sqrt_d) / a;
            if !ray_interval.surrounds(root) {
                return None;
            }
        }
        let point = ray.at(root);
        let outward_normal = (point - current_center) / self.radius;

        let rec = HitRecord::new(point, root, ray, outward_normal, self.material.clone());

        Some(rec)
    }
}

unsafe impl Sync for Sphere {}
