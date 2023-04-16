use std::cmp::Ordering;
use std::rc::Rc;

use rand::rngs::ThreadRng;
use rand::Rng;

use super::hittable::{HitRecord, Hittable, HittableList};
use crate::aabb::{surrounding_box, AABB};

pub struct BVHNode {
    left: Rc<Box<dyn Hittable>>,
    right: Rc<Box<dyn Hittable>>,
    r#box: AABB,
}

impl BVHNode {
    pub fn new(
        hittable_list: &mut HittableList,
        time0: f64,
        time1: f64,
    ) -> Self {
        let obj_len = hittable_list.objects.len();
        Self::new2(&mut hittable_list.objects, 0, obj_len, time0, time1)
    }

    pub fn new2(
        src_objects: &mut [Rc<Box<dyn Hittable>>],
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let mut rng = rand::thread_rng();

        let axis: usize = rng.gen_range(0..=2);
        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };

        let object_span = end - start;

        let left: Rc<Box<dyn Hittable>>;
        let right: Rc<Box<dyn Hittable>>;

        if object_span == 1 {
            left = Rc::clone(&src_objects[start]);
            right = Rc::clone(&src_objects[start]);
        } else if object_span == 2 {
            match comparator(&src_objects[start], &src_objects[start + 1]) {
                Ordering::Less => {
                    left = Rc::clone(&src_objects[start]);
                    right = Rc::clone(&src_objects[start + 1]);
                }
                _ => {
                    left = Rc::clone(&src_objects[start + 1]);
                    right = Rc::clone(&src_objects[start]);
                }
            }
        } else {
            src_objects[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            left = Rc::new(Box::new(Self::new2(
                src_objects,
                start,
                mid,
                time0,
                time1,
            )));
            right = Rc::new(Box::new(Self::new2(
                src_objects,
                mid,
                end,
                time0,
                time1,
            )));
        }

        let mut box_left = AABB::default();
        let mut box_right = AABB::default();

        if !left.bounding_box(time0, time1, &mut box_left)
            || !right.bounding_box(time0, time1, &mut box_right)
        {
            eprintln!("No bounding box in BVHNode constructor.");
        }

        Self {
            left,
            right,
            r#box: surrounding_box(&box_left, &box_right),
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.r#box.hit(ray, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(ray, t_min, t_max, rec);
        let t_max = if hit_left {
            rec.t
        } else {
            t_max
        };
        let hit_right = self.right.hit(ray, t_min, t_max, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.r#box;
        true
    }
}

fn box_x_compare(a: &Rc<Box<dyn Hittable>>, b: &Rc<Box<dyn Hittable>>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Rc<Box<dyn Hittable>>, b: &Rc<Box<dyn Hittable>>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Rc<Box<dyn Hittable>>, b: &Rc<Box<dyn Hittable>>) -> Ordering {
    box_compare(a, b, 2)
}

fn box_compare(a: &Rc<Box<dyn Hittable>>, b: &Rc<Box<dyn Hittable>>, axis: usize) -> Ordering {
    let mut box_a = AABB::default();
    let mut box_b = AABB::default();

    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        eprintln!("No bounding box in bvh_node constructor.");
    }

    box_a.min()[axis]
        .partial_cmp(&box_b.min()[axis])
        .unwrap_or(Ordering::Equal)
}

#[macro_export]
macro_rules! rc_box_bvh_node {
    ( $hitlist:expr, $time0:expr, $time1:expr ) => {
        Rc::new(Box::new(BVHNode::new(
            $hitlist,
            $time0,
            $time1
        )))
    };
}
