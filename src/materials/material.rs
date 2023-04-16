use rand::prelude::*;
use std::rc::Rc;

use crate::{
    hittables::hittable::HitRecord,
    ray::Ray,
    textures::texture::{SolidColor, Texture},
    vec3::{dot, random_in_unit_sphere, random_unit_vector, reflect, refract, Color, Point3},
};

pub trait Material {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color::with_value(0.0)
    }
}
