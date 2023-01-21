use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3};

pub struct Sphere {
    center: Point3,
    radius: f32,
    mat_ptr: Rc<Box<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, mat_ptr: &Rc<Box<dyn Material>>) -> Self {
        Self { center, radius, mat_ptr: Rc::clone(mat_ptr) }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
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
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        rec.mat_ptr = Rc::clone(&self.mat_ptr);

        true
    }
}
