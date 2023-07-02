use std::sync::Arc;

use crate::{
    aabb::AABB, materials::material::Material, rc_box_xy_rect, rc_box_xz_rect, rc_box_yz_rect,
    vec3::Point3,
};

use super::{
    hittable::{Hittable, HittableList},
    rect::{xy_rect::XYRect, xz_rect::XZRect, yz_rect::YZRect},
};

pub struct Cuboid {
    cuboid_min: Point3,
    cuboid_max: Point3,
    sides: HittableList,
}

impl Cuboid {
    pub fn new(p0: Point3, p1: Point3, mat_ptr: &Arc<Box<dyn Material>>) -> Self {
        let mut sides = HittableList::new();
        sides.add(rc_box_xy_rect!(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            mat_ptr
        ));
        sides.add(rc_box_xy_rect!(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            mat_ptr
        ));

        sides.add(rc_box_xz_rect!(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            mat_ptr
        ));
        sides.add(rc_box_xz_rect!(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            mat_ptr
        ));

        sides.add(rc_box_yz_rect!(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            mat_ptr
        ));
        sides.add(rc_box_yz_rect!(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            mat_ptr
        ));

        Self {
            cuboid_min: p0,
            cuboid_max: p1,
            sides,
        }
    }
}

impl Hittable for Cuboid {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut super::hittable::HitRecord,
    ) -> bool {
        self.sides.hit(ray, t_min, t_max, rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut crate::aabb::AABB) -> bool {
        *output_box = AABB::new(self.cuboid_min, self.cuboid_max);
        true
    }
}

#[macro_export]
macro_rules! rc_box_cuboid {
    ( $p0:expr, $p1:expr, $mat_ptr:expr ) => {
        Arc::new(Box::new(Cuboid::new($p0, $p1, $mat_ptr)))
    };
}
