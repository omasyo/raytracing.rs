use glam::Vec3;

pub mod checker_texture;
pub mod image_texture;
pub mod noise_texture;

pub trait Texture : Sync + Send {
   fn value(&self, u: f32, v: f32, point: Vec3) -> Vec3;
}