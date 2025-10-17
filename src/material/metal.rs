use glam::Vec3;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::utils::random_unit_vector;

pub struct Metal{
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        assert!(0.0 <= fuzz && fuzz <= 1.0);
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let mut reflected = r_in.direction.reflect(rec.normal);
        if self.fuzz > 0.0 {
            reflected = reflected.normalize() + (self.fuzz * random_unit_vector());
        }

        let result = ScatterResult {
            scattered: Ray::new(rec.point, reflected, r_in.time),
            attenuation: self.albedo,
        };
        Some(result)
    }
}