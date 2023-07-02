use std::sync::Arc;

use super::material::Material;
use crate::{
    hittables::hittable::HitRecord,
    ray::Ray,
    textures::texture::{SolidColor, Texture},
    vec3::{Color, Point3},
};

pub struct DiffuseLight {
    pub emit: Arc<Box<dyn Texture>>,
}

impl DiffuseLight {
    pub fn new(emit: &Arc<Box<dyn Texture>>) -> Self {
        Self {
            emit: Arc::clone(emit),
        }
    }

    pub fn with_color(color: Color) -> Self {
        Self {
            emit: Arc::new(Box::new(SolidColor::new(color))),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _: &Ray,
        _: &HitRecord,
        _: &mut Color,
        _: &mut Ray,
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
        Arc::new(Box::new(DiffuseLight::with_color(Color::with_value($rgb))))
    };
    ( $red:literal, $green:literal, $blue:literal ) => {
        Arc::new(Box::new(DiffuseLight::with_color(Color::with_values(
            $red, $green, $blue,
        ))))
    };
    ( Color, $color:expr ) => {
        Arc::new(Box::new(DiffuseLight::with_color($color)))
    };
    ( $material:expr ) => {
        Arc::new(Box::new(DiffuseLight::new($material)))
    };
}
