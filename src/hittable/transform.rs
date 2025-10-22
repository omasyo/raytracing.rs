use crate::hittable::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use glam::{vec3, Vec3};
use std::ops::Add;
use std::sync::Arc;

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bounding_box: Aabb,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bounding_box = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bounding_box,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_interval: Interval) -> Option<HitRecord> {
        let offset_ray = Ray::new(r.origin - self.offset, r.direction, r.time);

        let Some(mut hit_record) = self.object.hit(&offset_ray, ray_interval) else {
            return None;
        };
        hit_record.point += self.offset;

        Some(hit_record)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    bounding_box: Aabb,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f32) -> Self {
        let angle = angle.to_radians();
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();

        let bbox = object.bounding_box();

        let mut min = Vec3::INFINITY;
        let mut max = Vec3::NEG_INFINITY;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f32;
                    let j = j as f32;
                    let k = k as f32;

                    let x = i * bbox.x.max + (1.0 - i) * bbox.x.min;
                    let y = j * bbox.y.max + (1.0 - j) * bbox.y.min;
                    let z = k * bbox.z.max + (1.0 - k) * bbox.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_y = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_y);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        Self {
            object,
            sin_theta,
            cos_theta,
            bounding_box: Aabb::from((min, max)),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_interval: Interval) -> Option<HitRecord> {
        let origin = vec3(
            (self.cos_theta * r.origin.x) - (self.sin_theta * r.origin.z),
            r.origin.y,
            (self.sin_theta * r.origin.x) + (self.cos_theta * r.origin.z),
        );

        let direction = vec3(
            (self.cos_theta * r.direction.x) - (self.sin_theta * r.direction.z),
            r.direction.y,
            (self.sin_theta * r.direction.x) + (self.cos_theta * r.direction.z),
        );

        let rotated_ray = Ray::new(origin, direction, r.time);

        let Some(mut rec) = self.object.hit(&rotated_ray, ray_interval) else {
            return None;
        };

        rec.point = vec3(
            (self.cos_theta * rec.point.x) + (self.sin_theta * rec.point.z),
            rec.point.y,
            (-self.sin_theta * rec.point.x) + (self.cos_theta * rec.point.z),
        );

        rec.normal = vec3(
            (self.cos_theta * rec.normal.x) + (self.sin_theta * rec.normal.z),
            rec.normal.y,
            (-self.sin_theta * rec.normal.x) + (self.cos_theta * rec.normal.z),
        );

        Some(rec)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}