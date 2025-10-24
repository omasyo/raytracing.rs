use crate::hittable::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::material::isotropic::Isotropic;
use crate::material::texture::Texture;
use crate::ray::Ray;
use glam::Vec3;
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f32,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f32, texture: Box<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -density.recip(),
            phase_function: Arc::new(Isotropic::from(texture)),
        }
    }

    pub fn from_color(boundary: Arc<dyn Hittable>, density: f32, albedo: Vec3) -> Self {
        Self {
            boundary,
            neg_inv_density: -density.recip(),
            phase_function: Arc::new(Isotropic::from(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let Some(mut rec1) = self.boundary.hit(r, Interval::UNIVERSE) else {
            return None;
        };
        let Some(mut rec2) = self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0000_1, f32::INFINITY))
        else {
            return None;
        };
        rec1.t = rec1.t.max(ray_t.min);
        rec2.t = rec2.t.min(ray_t.max);

        if rec1.t >= rec2.t {
            return None;
        }

        rec1.t = rec1.t.max(0.0);

        let ray_length = r.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rand::random_range(0.0f32..=1.0).log2();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;

        let rec = HitRecord {
            point: r.at(t),
            normal: Vec3::X,
            material: self.phase_function.clone(),
            t,
            u: 0.0,
            v: 0.0,
            front_face: true,
        };

        Some(rec)
    }

    fn bounding_box(&self) -> &Aabb {
        self.boundary.bounding_box()
    }
}
