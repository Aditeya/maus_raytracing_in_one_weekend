use std::rc::Rc;

use super::super::hittable::{HitRecord, Hittable};
use crate::{
    aabb::AABB,
    materials::material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct XYRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    mat_ptr: Rc<Box<dyn Material>>,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: &Rc<Box<dyn Material>>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            mat_ptr: Rc::clone(mp),
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin().z()) / ray.direction().z();
        if t < t_min || t > t_max {
            return false;
        }

        let x = ray.origin().x() + t * ray.direction().x();
        let y = ray.origin().y() + t * ray.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;

        let outward_normal = Vec3::with_values(0.0, 0.0, 1.0);
        rec.set_face_normal(ray, &outward_normal);
        rec.mat_ptr = Rc::clone(&self.mat_ptr);
        rec.p = ray.at(t);
        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let min = Point3::with_values(self.x0, self.y0, self.k - 0.0001);
        let max = Point3::with_values(self.x1, self.y1, self.k + 0.0001);
        *output_box = AABB::new(min, max);
        true
    }
}

#[macro_export]
macro_rules! rc_box_xy_rect {
    ( $x0:literal, $x1:literal, $y0:literal, $y1:literal, $k:literal, $mat_ptr:expr ) => {
        Rc::new(Box::new(XYRect::new($x0, $x1, $y0, $y1, $k, $mat_ptr)))
    };
    ( $x0:expr, $x1:expr, $y0:expr, $y1:expr, $k:expr, $mat_ptr:expr ) => {
        Rc::new(Box::new(XYRect::new($x0, $x1, $y0, $y1, $k, $mat_ptr)))
    };
}
