use glam::Vec3;

pub trait Texture : Sync + Send {
   fn value(&self, u: f32, v: f32, point: Vec3) -> Vec3;
}