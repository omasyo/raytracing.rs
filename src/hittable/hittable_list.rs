use super::{HitRecord, Hittable};
use crate::hittable::aabb::Aabb;
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
    bounding_box: Aabb,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: vec![],
            bounding_box: Aabb::EMPTY,
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.bounding_box = Aabb::from((&self.bounding_box, hittable.bounding_box()));
        self.objects.push(hittable)
    }

    pub fn into_objects(self) -> Vec<Box<dyn Hittable>> {
        self.objects
    }
}

impl From<Box<dyn Hittable>> for HittableList {
    fn from(hittable: Box<dyn Hittable>) -> Self {
        let mut list = Self::new();
        list.add(hittable);
        list
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_interval: Interval) -> Option<HitRecord<'_>> {
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

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}
