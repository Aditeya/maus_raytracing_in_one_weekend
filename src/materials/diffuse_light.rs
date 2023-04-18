use std::rc::Rc;

use rand::rngs::ThreadRng;

use super::material::Material;
use crate::{
    hittables::hittable::HitRecord,
    ray::Ray,
    textures::texture::{SolidColor, Texture},
    vec3::{Color, Point3},
};

pub struct DiffuseLight {
    pub emit: Rc<Box<dyn Texture>>,
}

impl DiffuseLight {
    pub fn new(emit: &Rc<Box<dyn Texture>>) -> Self {
        Self {
            emit: Rc::clone(emit),
        }
    }

    pub fn with_color(color: Color) -> Self {
        Self {
            emit: Rc::new(Box::new(SolidColor::new(color))),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}

#[macro_export]
macro_rules! rc_box_diffuse_light {
    ( $rgb:literal ) => {
        Rc::new(Box::new(DiffuseLight::with_color(
            Color::with_value($rgb)
        )))
    };
    ( $red:literal, $green:literal, $blue:literal ) => {
        Rc::new(Box::new(DiffuseLight::with_color(
            Color::with_values($red, $green, $blue)
        )))
    };
    ( Color, $color:expr ) => {
        Rc::new(Box::new(DiffuseLight::with_color(
            $color
        )))
    };
    ( $material:expr ) => {
        Rc::new(Box::new(DiffuseLight::new(
            $material
        )))
    };
}
