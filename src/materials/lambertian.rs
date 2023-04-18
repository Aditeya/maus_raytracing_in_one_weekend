use std::rc::Rc;

use rand::rngs::ThreadRng;

use crate::{
    hittables::hittable::HitRecord,
    ray::Ray,
    textures::texture::{SolidColor, Texture},
    vec3::{random_unit_vector, Color},
};

use super::material::Material;

pub struct Lambertian {
    pub albedo: Rc<Box<dyn Texture>>,
}

impl Lambertian {
    pub fn new(albedo: &Rc<Box<dyn Texture>>) -> Self {
        Self {
            albedo: Rc::clone(albedo),
        }
    }

    pub fn with_color(albedo: Color) -> Self {
        Self {
            albedo: Rc::new(Box::new(SolidColor::new(albedo))),
        }
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Self {
            albedo: Rc::new(Box::new(SolidColor::with_values(1.0, 1.0, 1.0))),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}

#[macro_export]
macro_rules! rc_box_lambertian {
    ( $rgb:literal ) => {
        Rc::new(Box::new(Lambertian::with_color(
            Color::with_value($rgb)
        )))
    };
    ( $red:literal, $green:literal, $blue:literal ) => {
        Rc::new(Box::new(Lambertian::with_color(
            Color::with_values($red, $green, $blue)
        )))
    };
    ( Color, $color:expr ) => {
        Rc::new(Box::new(Lambertian::with_color(
            $color
        )))
    };
    ( $material:expr ) => {
        Rc::new(Box::new(Lambertian::new(
            $material
        )))
    };
}
