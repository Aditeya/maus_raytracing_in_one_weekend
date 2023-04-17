use std::rc::Rc;

use super::hittable::{HitRecord, Hittable};
use crate::{
    aabb::{surrounding_box, AABB},
    materials::material::Material,
    ray::Ray,
    vec3::{dot, Point3, Vec3},
};

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    mat_ptr: Rc<Box<dyn Material>>,
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        mat_ptr: &Rc<Box<dyn Material>>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            mat_ptr: Rc::clone(mat_ptr),
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center(ray.time());
        let dir = ray.direction();

        let a = dir.length_squared();
        let half_b = dot(&oc, &dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // NOTE: Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center(ray.time())) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        rec.mat_ptr = Rc::clone(&self.mat_ptr);

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let r_vec = Vec3::with_value(self.radius);

        let center = self.center(time0);
        let box0 = AABB::new(
            center - r_vec,
            center + r_vec,
        );

        let center = self.center(time1);
        let box1 = AABB::new(
            center - r_vec,
            center + r_vec,
        );

        *output_box = surrounding_box(&box0, &box1);
        true
    }
}

#[macro_export]
macro_rules! rc_box_moving_sphere {
    ( $center1:expr, $center2:expr, $time0:literal, $time1:literal, $radius:literal, $mat_ptr:expr ) => {
        Rc::new(Box::new(MovingSphere::new(
            $center1,
            $center2,
            $time0,
            $time1,
            $radius,
            $mat_ptr
        )))
    };
}
