use std::mem::swap;

use crate::{ray::Ray, vec3::Point3};

#[derive(Default, Copy, Clone)]
pub struct AABB {
    minimum: Point3,
    maximum: Point3,
}

impl AABB {
    pub fn new(minimum: Point3, maximum: Point3) -> Self {
        Self { minimum, maximum }
    }

    pub fn min(&self) -> Point3 {
        self.minimum
    }

    pub fn max(&self) -> Point3 {
        self.maximum
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction()[a];
            let mut t0 = (self.minimum[a] - ray.origin()[a]) * inv_d;
            let mut t1 = (self.maximum[a] - ray.origin()[a]) * inv_d;

            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }

            if t0 > t_min {
                t_min = t0;
            }

            if t1 < t_max {
                t_max = t1;
            }

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Point3::with_values(
        box0.minimum.x().min(box1.minimum.x()),
        box0.minimum.y().min(box1.minimum.y()),
        box0.minimum.z().min(box1.minimum.z()),
    );

    let big = Point3::with_values(
        box0.maximum.x().max(box1.maximum.x()),
        box0.maximum.y().max(box1.maximum.y()),
        box0.maximum.z().max(box1.maximum.z()),
    );

    AABB::new(small, big)
}
