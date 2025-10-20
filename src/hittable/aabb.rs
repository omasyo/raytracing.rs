use crate::interval::Interval;
use crate::ray::Ray;
use glam::Vec3;

pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }
    pub fn hit(&self, ray: &Ray, mut ray_t: Interval) -> bool {
        for axis in 0..3 {
            let interval = self.axis_interval(axis);
            let axis_dir_inv = 1.0 / ray.direction[axis];

            let t0 = (interval.min - ray.origin[axis]) * axis_dir_inv;
            let t1 = (interval.max - ray.origin[axis]) * axis_dir_inv;

            if t0 < t1 {
                ray_t.min = t0.max(ray_t.min);
                ray_t.max = t1.min(ray_t.max);
            } else {
                ray_t.min = t1.max(ray_t.min);
                ray_t.max = t0.min(ray_t.max);
            }
            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("invalid interval {}", n),
        }
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 1 }
        } else {
            if self.y.size() > self.z.size() { 1 } else { 2 }
        }
    }

    pub const EMPTY: Self = Self {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    pub const UNIVERSE: Self = Self {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };
}

impl From<(Vec3, Vec3)> for Aabb {
    fn from((a, b): (Vec3, Vec3)) -> Self {
        Self {
            x: if a.x <= b.x {
                Interval::new(a.x, b.x)
            } else {
                Interval::new(b.x, a.x)
            },
            y: if a.y <= b.y {
                Interval::new(a.y, b.y)
            } else {
                Interval::new(b.y, a.y)
            },
            z: if a.z <= b.z {
                Interval::new(a.z, b.z)
            } else {
                Interval::new(b.z, a.z)
            },
        }
    }
}

impl From<(&Aabb, &Aabb)> for Aabb {
    fn from((a, b): (&Aabb, &Aabb)) -> Self {
        Self {
            x: Interval::from((&a.x, &b.x)),
            y: Interval::from((&a.y, &b.y)),
            z: Interval::from((&a.z, &b.z)),
        }
    }
}
