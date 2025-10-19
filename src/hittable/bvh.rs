use crate::hittable::aabb::Aabb;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use glam::Vec3;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bounding_box: Aabb,
}

impl BvhNode {
    pub fn new(objects: &mut [Arc<dyn Hittable>]) -> Self {
        let axis = rand::random_range(0..=2);

        let comparator: fn(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering = match axis {
            0 => |a, b| box_compare(a, b, 0),
            1 => |a, b| box_compare(a, b, 1),
            2 => |a, b| box_compare(a, b, 2),
            _ => panic!("invalid axis: {axis}"),
        };

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

        match objects.len() {
            1 => {
                left = objects[0].clone();
                right = left.clone();
            }
            2 => {
                left = objects[0].clone();
                right = objects[1].clone();
            }
            _ => {
                objects.sort_by(comparator);
                let mid = objects.len() / 2;
                left = Arc::new(Self::new(objects[..mid].as_mut()));
                right = Arc::new(Self::new(objects[mid..].as_mut()))
            }
        }

        let bounding_box = Aabb::from((left.bounding_box(), right.bounding_box()));
        Self {
            left,
            right,
            bounding_box,
        }
    }
}

impl From<HittableList> for BvhNode {
    fn from(mut list: HittableList) -> Self {
        Self::new(list.objects.as_mut_slice())
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bounding_box.hit(r, ray_t) {
            return None;
        }
        let hit_left = self.left.hit(r, ray_t);
        let end = if let Some(HitRecord { t, .. }) = hit_left {
            t
        } else {
            ray_t.max
        };
        let hit_right = self.right.hit(r, Interval::new(ray_t.min, end));

        hit_right.or(hit_left)
    }
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: usize) -> Ordering {
    let a_axis_interval = a.bounding_box().axis_interval(axis_index);
    let b_axis_interval = b.bounding_box().axis_interval(axis_index);
    // This is a safer way to compare floats
    a_axis_interval
        .min
        .partial_cmp(&b_axis_interval.min)
        .unwrap_or(Ordering::Equal)
}
