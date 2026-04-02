pub mod texture;

use crate::hittable::HitRecord;
use crate::material::texture::TextureKind;
use crate::ray::Ray;
use crate::utils::{near_zero, random_unit_vector};
use glam::{vec3, Vec3};

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Vec3,
}

#[derive(Clone)]
pub enum MaterialKind {
    Lambertian { texture: TextureKind },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { refraction_index: f32 },
    DiffuseLight { texture: TextureKind },
    Isotropic { texture: TextureKind },
}

impl MaterialKind {
    pub fn lambertian(color: Vec3) -> Self {
        Self::Lambertian {
            texture: TextureKind::solid(color),
        }
    }

    pub fn lambertian_textured(texture: TextureKind) -> Self {
        Self::Lambertian { texture }
    }

    pub fn metal(albedo: Vec3, fuzz: f32) -> Self {
        assert!((0.0..=1.0).contains(&fuzz));
        Self::Metal { albedo, fuzz }
    }

    pub fn dielectric(refraction_index: f32) -> Self {
        Self::Dielectric { refraction_index }
    }

    pub fn diffuse_light(color: Vec3) -> Self {
        Self::DiffuseLight {
            texture: TextureKind::solid(color),
        }
    }

    pub fn diffuse_light_textured(texture: TextureKind) -> Self {
        Self::DiffuseLight { texture }
    }

    pub fn isotropic(color: Vec3) -> Self {
        Self::Isotropic {
            texture: TextureKind::solid(color),
        }
    }

    pub fn isotropic_textured(texture: TextureKind) -> Self {
        Self::Isotropic { texture }
    }

    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        match self {
            Self::Lambertian { texture } => {
                let mut scatter_direction = rec.normal + random_unit_vector();
                if near_zero(scatter_direction) {
                    scatter_direction = rec.normal;
                }
                Some(ScatterResult {
                    scattered: Ray::new(rec.point, scatter_direction, r_in.time),
                    attenuation: texture.value(rec.u, rec.v, rec.point),
                })
            }
            Self::Metal { albedo, fuzz } => {
                let mut reflected = r_in.direction.reflect(rec.normal);
                if *fuzz > 0.0 {
                    reflected = reflected.normalize() + (*fuzz * random_unit_vector());
                }
                Some(ScatterResult {
                    scattered: Ray::new(rec.point, reflected, r_in.time),
                    attenuation: *albedo,
                })
            }
            Self::Dielectric { refraction_index } => {
                let ri = if rec.front_face {
                    refraction_index.recip()
                } else {
                    *refraction_index
                };
                let unit_direction = r_in.direction.normalize();
                let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let direction =
                    if (ri * sin_theta > 1.0) || (reflectance(cos_theta, ri) > rand::random())
                    {
                        unit_direction.reflect(rec.normal)
                    } else {
                        unit_direction.refract(rec.normal, ri)
                    };
                Some(ScatterResult {
                    scattered: Ray::new(rec.point, direction, r_in.time),
                    attenuation: vec3(1.0, 1.0, 1.0),
                })
            }
            Self::DiffuseLight { .. } => None,
            Self::Isotropic { texture } => Some(ScatterResult {
                scattered: Ray::new(rec.point, random_unit_vector(), r_in.time),
                attenuation: texture.value(rec.u, rec.v, rec.point),
            }),
        }
    }

    pub fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        match self {
            Self::DiffuseLight { texture } => texture.value(u, v, p),
            _ => Vec3::ZERO,
        }
    }
}

fn reflectance(cosine: f32, refraction_idx: f32) -> f32 {
    let mut r0 = (1.0 - refraction_idx) / (1.0 + refraction_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
