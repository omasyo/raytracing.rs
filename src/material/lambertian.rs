use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::solid_color::SolidColor;
use crate::material::texture::Texture;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::utils::{near_zero, random_unit_vector};
use glam::Vec3;

pub struct Lambertian {
    texture: Box<dyn Texture>,
}

impl From<Vec3> for Lambertian {
    fn from(color: Vec3) -> Self {
        Self {
            texture: Box::new(SolidColor::new(&Color::new(color))),
        }
    }
}

impl From<&Color> for Lambertian {
    fn from(color: &Color) -> Self {
        Self {
            texture: Box::new(SolidColor::new(color)),
        }
    }
}

impl From<Box<dyn Texture>> for Lambertian {
    fn from(texture: Box<dyn Texture>) -> Self {
        Self { texture }
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
            attenuation: self.texture.value(rec.u, rec.v, rec.point),
        };
        Some(result)
    }
}
