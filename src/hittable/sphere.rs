use super::{HitRecord, Hittable};
use crate::hittable::aabb::Aabb;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3;
use std::f32::consts::PI;
use std::sync::Arc;

pub struct Sphere {
    center: Ray,
    radius: f32,
    material: Arc<dyn Material>,
    bounding_box: Aabb,
}

impl Sphere {
    pub fn new_stationary(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Sphere {
        assert!(radius > 0.0);
        let r_vec = Vec3::new(radius, radius, radius);
        Sphere {
            center: Ray::new(center, Vec3::ZERO, 0.0),
            radius,
            material,
            bounding_box: Aabb::from((center - r_vec, center + r_vec)),
        }
    }

    pub fn new_moving(
        center1: Vec3,
        center2: Vec3,
        radius: f32,
        material: Arc<dyn Material>,
    ) -> Sphere {
        assert!(radius > 0.0);

        let center = Ray::new(center1, center2 - center1, 0.0);
        let r_vec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::from((center.at(0.0) - r_vec, center.at(0.0) + r_vec));
        let box2 = Aabb::from((center.at(1.0) - r_vec, center.at(1.0) + r_vec));
        Sphere {
            center: Ray::new(center1, center2 - center1, 0.0),
            radius,
            material,
            bounding_box: Aabb::from((&box1, &box2)),
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

        let rec = HitRecord::new(
            point,
            root,
            ray,
            outward_normal,
            self.material.clone(),
            get_sphere_uv(outward_normal),
        );

        Some(rec)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}

fn get_sphere_uv(p: Vec3) -> (f32, f32) {
    let theta = (-p.y.clamp(-1.0, 1.0)).acos();
    let phi = (-p.z).atan2(p.x) + PI;

    let u = phi / (2.0 * PI);
    let v = theta / PI;
    (u, v)
}
