use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::solid_color::SolidColor;
use crate::material::texture::Texture;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::utils::random_unit_vector;
use glam::Vec3;

pub struct Isotropic {
    texture: Box<dyn Texture>,
}

impl From<Vec3> for Isotropic {
    fn from(color: Vec3) -> Self {
        Self {
            texture: Box::new(SolidColor::new(&Color::new(color))),
        }
    }
}

impl From<&Color> for Isotropic {
    fn from(color: &Color) -> Self {
        Self {
            texture: Box::new(SolidColor::new(color)),
        }
    }
}

impl From<Box<dyn Texture>> for Isotropic {
    fn from(texture: Box<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let res = ScatterResult {
            scattered: Ray::new(rec.point, random_unit_vector(), r_in.time),
            attenuation: self.texture.value(rec.u, rec.v, rec.point),
        };
        Some(res)
    }
}
