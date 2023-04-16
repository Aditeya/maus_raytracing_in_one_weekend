use std::rc::Rc;

use crate::{aabb::AABB, ray::Ray, vec3::Vec3};

use super::hittable::{HitRecord, Hittable};

pub struct Translate {
    ptr: Rc<Box<dyn Hittable>>,
    offset: Vec3,
}

impl Translate {
    pub fn new(p: &Rc<Box<dyn Hittable>>, displacement: Vec3) -> Self {
        Self {
            ptr: Rc::clone(p),
            offset: displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());
        if !self.ptr.hit(&moved_r, t_min, t_max, rec) {
            return false;
        }

        rec.p += self.offset;
        let outward_normal = rec.normal;
        rec.set_face_normal(&moved_r, &outward_normal);

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if !self.ptr.bounding_box(time0, time1, output_box) {
            return false;
        }

        *output_box = AABB::new(
            output_box.min() + self.offset,
            output_box.max() + self.offset,
        );
        true
    }
}

#[macro_export]
macro_rules! rc_box_translate {
    ( $ptr:expr, $displacement:expr ) => {
        Rc::new(Box::new(Translate::new($ptr, $displacement)))
    };
}
