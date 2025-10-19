use std::sync::Arc;
use super::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, hittable: Arc<dyn Hittable>) {
        self.objects.push(hittable)
    }
}

impl From<Vec<Arc<dyn Hittable>>> for HittableList {
    fn from(value: Vec<Arc<dyn Hittable>>) -> Self {
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