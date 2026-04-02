use crate::hittable::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::MaterialKind;
use crate::material::texture::TextureKind;
use crate::ray::Ray;
use glam::Vec3;

pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    neg_inv_density: f32,
    phase_function: MaterialKind,
}

impl ConstantMedium {
    pub fn new(boundary: Box<dyn Hittable>, density: f32, texture: TextureKind) -> Self {
        Self {
            boundary,
            neg_inv_density: -density.recip(),
            phase_function: MaterialKind::isotropic_textured(texture),
        }
    }

    pub fn from_color(boundary: Box<dyn Hittable>, density: f32, albedo: Vec3) -> Self {
        Self {
            boundary,
            neg_inv_density: -density.recip(),
            phase_function: MaterialKind::isotropic(albedo),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
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

        Some(HitRecord {
            point: r.at(t),
            normal: Vec3::X,
            material: &self.phase_function,
            t,
            u: 0.0,
            v: 0.0,
            front_face: true,
        })
    }

    fn bounding_box(&self) -> &Aabb {
        self.boundary.bounding_box()
    }
}
