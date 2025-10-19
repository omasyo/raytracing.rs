use crate::hittable::HitRecord;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use glam::vec3;

pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let refractive_index = if rec.front_face {
            self.refraction_index.recip()
        } else {
            self.refraction_index
        };
        let unit_direction = r_in.direction.normalize();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if (refractive_index * sin_theta > 1.0)
            || (reflectance(cos_theta, refractive_index) > rand::random_range(0.0..=1.0))
        {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refractive_index)
        };

        let result = ScatterResult {
            scattered: Ray::new(rec.point, direction, r_in.time),
            attenuation: vec3(1.0, 1.0, 1.0),
        };
        Some(result)
    }
}

fn reflectance(cosine: f32, refraction_idx: f32) -> f32 {
    let mut r0 = (1.0 - refraction_idx) / (1.0 + refraction_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
