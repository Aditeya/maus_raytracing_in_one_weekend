use std::f64::consts::PI;
use std::sync::Arc;

use super::hittable::{HitRecord, Hittable};
use crate::{
    aabb::AABB,
    materials::material::Material,
    ray::Ray,
    vec3::{dot, Point3, Vec3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Arc<Box<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: &Arc<Box<dyn Material>>) -> Self {
        Self {
            center,
            radius,
            mat_ptr: Arc::clone(mat_ptr),
        }
    }

    fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
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
        Sphere::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.mat_ptr = Arc::clone(&self.mat_ptr);

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            self.center - Vec3::with_value(self.radius),
            self.center + Vec3::with_value(self.radius),
        );

        true
    }
}

#[macro_export]
macro_rules! rc_box_sphere {
    ( $point:expr, $radius:literal, $mat_ptr:expr ) => {
        Arc::new(Box::new(Sphere::new($point, $radius, $mat_ptr)))
    };
}
