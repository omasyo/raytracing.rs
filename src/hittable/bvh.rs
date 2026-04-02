use crate::hittable::aabb::Aabb;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use std::cmp::Ordering;

pub struct BvhNode {
    left: Box<dyn Hittable>,
    right: Option<Box<dyn Hittable>>,
    bounding_box: Aabb,
}

impl BvhNode {
    pub fn new(mut objects: Vec<Box<dyn Hittable>>) -> Self {
        let mut bounding_box = Aabb::EMPTY;
        for object in objects.iter() {
            bounding_box = Aabb::from((&bounding_box, object.bounding_box()));
        }

        let axis = bounding_box.longest_axis();

        let comparator: fn(&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering = match axis {
            0 => |a, b| box_compare(a.as_ref(), b.as_ref(), 0),
            1 => |a, b| box_compare(a.as_ref(), b.as_ref(), 1),
            2 => |a, b| box_compare(a.as_ref(), b.as_ref(), 2),
            _ => panic!("Invalid axis: {axis}"),
        };

        match objects.len() {
            0 => panic!("BvhNode::new called with empty objects"),
            1 => {
                Self {
                    left: objects.pop().unwrap(),
                    right: None,
                    bounding_box,
                }
            }
            2 => {
                objects.sort_by(comparator);
                let right = objects.pop().unwrap();
                let left = objects.pop().unwrap();
                Self {
                    left,
                    right: Some(right),
                    bounding_box,
                }
            }
            _ => {
                objects.sort_by(comparator);
                let mid = objects.len() / 2;
                let right_objects = objects.split_off(mid);
                let left: Box<dyn Hittable> = Box::new(Self::new(objects));
                let right: Box<dyn Hittable> = Box::new(Self::new(right_objects));
                Self {
                    left,
                    right: Some(right),
                    bounding_box,
                }
            }
        }
    }
}

impl From<HittableList> for BvhNode {
    fn from(list: HittableList) -> Self {
        Self::new(list.into_objects())
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        if !self.bounding_box.hit(r, ray_t) {
            return None;
        }

        let hit_left = self.left.hit(r, ray_t);

        if let Some(right) = &self.right {
            let end = hit_left.as_ref().map_or(ray_t.max, |rec| rec.t);
            let hit_right = right.hit(r, Interval::new(ray_t.min, end));
            hit_right.or(hit_left)
        } else {
            hit_left
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}

fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis_index: usize) -> Ordering {
    let a_axis_interval = a.bounding_box().axis_interval(axis_index);
    let b_axis_interval = b.bounding_box().axis_interval(axis_index);
    a_axis_interval
        .min
        .partial_cmp(&b_axis_interval.min)
        .unwrap_or(Ordering::Equal)
}
