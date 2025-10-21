use crate::hittable::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3;
use std::sync::Arc;

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    material: Arc<dyn Material>,
    bounding_box: Aabb,
    normal: Vec3,
    d: f32,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        let n = u.cross(v);
        let normal = n.normalize();

        let bbox_diagonal1 = Aabb::from((q, q + u + v));
        let bbox_diagonal2 = Aabb::from((q + u, q + v));
        let bounding_box = Aabb::from((&bbox_diagonal1, &bbox_diagonal2));

        Self {
            q,
            u,
            v,
            material,
            bounding_box,
            normal,
            d: normal.dot(q),
            w: n / n.dot(n),
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_interval: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direction);

        if denom.abs() < f32::EPSILON {
            return None;
        }

        let t = (self.d - self.normal.dot(ray.origin)) / denom;
        if !ray_interval.contains(t) {
            return None;
        }

        let intersection = ray.at(t);
        let planar_hit_pt_vector = intersection - self.q;
        let alpha = self.w.dot(planar_hit_pt_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hit_pt_vector));

        let Some(plane_coord) = is_interior(alpha, beta) else {
            return None;
        };

        let rec = HitRecord::new(
            intersection,
            t,
            ray,
            self.normal,
            self.material.clone(),
            plane_coord,
        );

        Some(rec)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}

fn is_interior(a: f32, b: f32) -> Option<(f32, f32)> {
    let unit_interval = Interval::new(0.0, 1.0);

    if !unit_interval.contains(a) || !unit_interval.contains(b) {
        return None;
    }

    Some((a, b))
}
