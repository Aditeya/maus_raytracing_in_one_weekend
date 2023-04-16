use std::rc::Rc;

use rand::Rng;

use crate::{materials::{material::Material, isotropic::Isotropic}, ray::Ray, aabb::AABB, vec3::{Color, Vec3}};

use super::hittable::{Hittable, HitRecord};

pub struct ConstantMedium {
    boundary: Rc<Box<dyn Hittable>>,
    phase_function: Rc<Box<dyn Material>>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(b: &Rc<Box<dyn Hittable>>, d: f64, a: &Rc<Box<dyn Material>>) -> Self {
        Self {
            boundary: Rc::clone(b),
            neg_inv_density: -1.0/d,
            phase_function: Rc::clone(a),
        }
    }

    pub fn with_color(b: &Rc<Box<dyn Hittable>>, d: f64, a: Color) -> Self {
        Self {
            boundary: Rc::clone(b),
            neg_inv_density: -1.0/d,
            phase_function: Rc::new(Box::new(Isotropic::new(a))),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut rng = rand::thread_rng();
        let enable_debug = false;
        let debugging = enable_debug && rng.gen_range(0.0..1.0) < 0.00001;

        let mut rec1: HitRecord = Default::default();
        let mut rec2: HitRecord = Default::default();

        if !self.boundary.hit(ray, -f64::INFINITY, f64::INFINITY, &mut rec1) {
            return false;
        }
        if !self.boundary.hit(ray, rec1.t + 0.0001, f64::INFINITY, &mut rec2) {
            return false;
        }

        if debugging {
            println!("\nt_min = {0}, t_max = {1}", rec1.t, rec2.t);
        }

        if rec1.t < t_min { rec1.t = t_min; }
        if rec2.t > t_max { rec2.t = t_max; }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = ray.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rng.gen_range(0.0..1.0f64).ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = ray.at(rec.t);

        if debugging {
            println!("hit_distance = {}\nnrec.t = {}\nnrec.p = {}", hit_distance, rec.t, rec.p);
        }

        rec.normal = Vec3::with_values(1.0, 0.0, 0.0); // arbitrary
        rec.front_face = true; // also arbitrary
        rec.mat_ptr = Rc::clone(&self.phase_function);

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
}

#[macro_export]
macro_rules! rc_box_constant_medium {
    ( $b:expr, $d:expr, $color:expr, Color ) => {
        Rc::new(Box::new(ConstantMedium::with_color(
            $b,
            $d,
            $color
        )))
    };
    ( $b:expr, $d:expr, $mat_ptr:expr ) => {
        Rc::new(Box::new(ConstantMedium::new(
            $b,
            $d,
            $mat_ptr
        )))
    };
}
