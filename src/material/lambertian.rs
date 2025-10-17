use crate::hittable::HitRecord;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::utils::{near_zero, random_unit_vector};
use glam::Vec3;

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }

        let result = ScatterResult {
            scattered: Ray::new(rec.point, scatter_direction, r_in.time),
            attenuation: self.albedo,
        };
        Some(result)
    }
}
