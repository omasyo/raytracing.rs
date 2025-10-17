use super::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Sync>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable + Sync>) {
        self.objects.push(hittable)
    }
}

impl From<Vec<Box<dyn Hittable + Sync>>> for HittableList {
    fn from(value: Vec<Box<dyn Hittable + Sync>>) -> Self {
        HittableList { objects: value }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_interval: Interval) -> Option<HitRecord> {
        // let mut temp_rec = HitRecord::default();
        let mut closest_so_far = ray_interval.max;
        let mut rec = None;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(ray, Interval::new(ray_interval.min, closest_so_far))
            {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }

        rec
    }
}

unsafe impl Send for HittableList {}