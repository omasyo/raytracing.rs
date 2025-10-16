use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::Ray;
use glam::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        assert!(radius > 0.0);
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_interval: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;

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
        let outward_normal = (point - self.center) / self.radius;

        let rec = HitRecord::new(point, root, ray, outward_normal);

        Some(rec)
    }
}
