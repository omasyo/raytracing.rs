pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use glam::Vec3;

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Vec3,
}

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult>;
}
