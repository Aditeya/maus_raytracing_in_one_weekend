use std::rc::Rc;

use crate::{
    aabb::AABB,
    hittables::hittable::{HitRecord, Hittable},
    materials::material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct XZRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mat_ptr: Rc<Box<dyn Material>>,
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mp: &Rc<Box<dyn Material>>) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            mat_ptr: Rc::clone(mp),
        }
    }
}

impl Hittable for XZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin().y()) / ray.direction().y();
        if t < t_min || t > t_max {
            return false;
        }

        let x = ray.origin().x() + t * ray.direction().x();
        let z = ray.origin().z() + t * ray.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;

        let outward_normal = Vec3::with_values(0.0, 1.0, 0.0);
        rec.set_face_normal(ray, &outward_normal);
        rec.mat_ptr = Rc::clone(&self.mat_ptr);
        rec.p = ray.at(t);
        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let min = Point3::with_values(self.x0, self.k - 0.0001, self.z0);
        let max = Point3::with_values(self.x1, self.k + 0.0001, self.z1);
        *output_box = AABB::new(min, max);
        true
    }
}

#[macro_export]
macro_rules! rc_box_xz_rect {
    ( $x0:literal, $x1:literal, $z0:literal, $z1:literal, $k:literal, $mat_ptr:expr ) => {
        Rc::new(Box::new(XZRect::new($x0, $x1, $z0, $z1, $k, $mat_ptr)))
    };
    ( $x0:expr, $x1:expr, $z0:expr, $z1:expr, $k:expr, $mat_ptr:expr ) => {
        Rc::new(Box::new(XZRect::new($x0, $x1, $z0, $z1, $k, $mat_ptr)))
    };
}
