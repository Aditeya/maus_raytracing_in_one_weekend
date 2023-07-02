use std::sync::Arc;

use crate::{
    hittables::hittable::HitRecord,
    ray::Ray,
    textures::texture::{SolidColor, Texture},
    vec3::{random_in_unit_sphere, Color},
};

use super::material::Material;

pub struct Isotropic {
    albedo: Arc<Box<dyn Texture>>,
}

impl Isotropic {
    pub fn new(color: Color) -> Self {
        Self {
            albedo: Arc::new(Box::new(SolidColor::new(color))),
        }
    }

    pub fn with_texture(a: &Arc<Box<dyn Texture>>) -> Self {
        Self {
            albedo: Arc::clone(a),
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
