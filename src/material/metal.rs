use glam::Vec3;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;

pub struct Metal{
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let reflected = r_in.direction.reflect(rec.normal);

        let result = ScatterResult {
            scattered: Ray::new(rec.point, reflected),
            attenuation: self.albedo,
        };
        Some(result)
    }
}