use std::rc::Rc;

use crate::{vec3::{Vec3, Point3}, aabb::AABB, ray::Ray};

use super::hittable::{Hittable, HitRecord};

pub struct RotateY {
    ptr: Rc<Box<dyn Hittable>>,
    sin_theta: f64,
    cos_theta: f64,
    has_box: bool,
    bbox: AABB,
}

impl RotateY {
    pub fn new(p: &Rc<Box<dyn Hittable>>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let mut bbox = Default::default();
        let has_box = p.bounding_box(0.0, 1.0, &mut bbox);

        let mut min = Point3::with_values(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::with_values(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;

                    let x = i*bbox.max().x() + (1.0-i)*bbox.min().x();
                    let y = j*bbox.max().y() + (1.0-j)*bbox.min().y();
                    let z = k*bbox.max().z() + (1.0-k)*bbox.min().z();

                    let newx = cos_theta*x + sin_theta*z;
                    let newz = -sin_theta*x + cos_theta*z;

                    let tester = Vec3::with_values(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        bbox = AABB::new(min, max);

        Self {
            ptr: Rc::clone(p),
            sin_theta,
            cos_theta,
            has_box,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = ray.origin();
        let mut direction = ray.direction();

        origin[0] = self.cos_theta*ray.origin()[0] - self.sin_theta*ray.origin()[2];
        origin[2] = self.sin_theta*ray.origin()[0] + self.cos_theta*ray.origin()[2];

        direction[0] = self.cos_theta*ray.direction()[0] - self.sin_theta*ray.direction()[2];
        direction[2] = self.sin_theta*ray.direction()[0] + self.cos_theta*ray.direction()[2];

        let rotated_ray = Ray::new(origin, direction, ray.time());

        if !self.ptr.hit(&rotated_ray, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p;
        let mut normal = rec.normal;

        p[0] = self.cos_theta*rec.p[0] + self.sin_theta*rec.p[2];
        p[2] = -self.sin_theta*rec.p[0] + self.cos_theta*rec.p[2];

        normal[0] = self.cos_theta*rec.normal[0] + self.sin_theta*rec.normal[2];
        normal[2] = -self.sin_theta*rec.normal[0] + self.cos_theta*rec.normal[2];

        rec.p = p;
        rec.set_face_normal(&rotated_ray, &normal);

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        self.has_box
    }
}

#[macro_export]
macro_rules! rc_box_rotate_y {
    ( $ptr:expr, $angle:expr ) => {
        Rc::new(Box::new(RotateY::new(
            $ptr,
            $angle
        )))
    };
}
