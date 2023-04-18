use std::rc::Rc;

use rand::rngs::ThreadRng;

use crate::{textures::texture::{Texture, SolidColor}, vec3::{Color, random_in_unit_sphere}, ray::Ray, hittables::hittable::HitRecord};

use super::material::Material;

pub struct Isotropic {
    albedo: Rc<Box<dyn Texture>>,
}

impl Isotropic {
    pub fn new(color: Color) -> Self {
        Self {
            albedo: Rc::new(Box::new(SolidColor::new(color))),
        }
    }

    pub fn with_texture(a: &Rc<Box<dyn Texture>>) -> Self {
        Self {
            albedo: Rc::clone(a),
        }
    }
}

impl Material for Isotropic {
    fn scatter(
            &self,
            r_in: &Ray,
            rec: &HitRecord,
            attenuation: &mut Color,
            scattered: &mut Ray,
        ) -> bool {
        *scattered = Ray::new(rec.p, random_in_unit_sphere(), r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}
