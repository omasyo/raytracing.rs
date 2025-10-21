pub mod dielectric;
pub mod lambertian;
pub mod metal;
pub mod texture;
pub mod solid_color;
pub mod diffuse_light;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use glam::Vec3;

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Vec3,
}

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        Vec3::ZERO
    }
}
