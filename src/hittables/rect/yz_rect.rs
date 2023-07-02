use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittables::hittable::{HitRecord, Hittable},
    materials::material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct YZRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mat_ptr: Arc<Box<dyn Material>>,
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mp: &Arc<Box<dyn Material>>) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            mat_ptr: Arc::clone(mp),
        }
    }
}

impl Hittable for YZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin().x()) / ray.direction().x();
        if t < t_min || t > t_max {
            return false;
        }

        let y = ray.origin().y() + t * ray.direction().y();
        let z = ray.origin().z() + t * ray.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }

        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;

        let outward_normal = Vec3::with_values(1.0, 0.0, 0.0);
        rec.set_face_normal(ray, &outward_normal);
        rec.mat_ptr = Arc::clone(&self.mat_ptr);
        rec.p = ray.at(t);
        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let min = Point3::with_values(self.k - 0.0001, self.y0, self.z0);
        let max = Point3::with_values(self.k + 0.0001, self.y1, self.z1);
        *output_box = AABB::new(min, max);
        true
    }
}

#[macro_export]
macro_rules! rc_box_yz_rect {
    ( $y0:literal, $y1:literal, $z0:literal, $z1:literal, $k:literal, $mat_ptr:expr ) => {
        Arc::new(Box::new(YZRect::new($y0, $y1, $z0, $z1, $k, $mat_ptr)))
    };
    ( $y0:expr, $y1:expr, $z0:expr, $z1:expr, $k:expr, $mat_ptr:expr ) => {
        Arc::new(Box::new(YZRect::new($y0, $y1, $z0, $z1, $k, $mat_ptr)))
    };
}
