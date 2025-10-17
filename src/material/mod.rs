pub mod lambertian;
pub mod metal;
pub mod dielectric;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use glam::Vec3;

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Vec3,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult>;
}
