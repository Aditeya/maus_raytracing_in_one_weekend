use rand::rngs::ThreadRng;

use crate::{
    hittables::hittable::HitRecord,
    ray::Ray,
    vec3::{dot, random_in_unit_sphere, reflect, Color},
};

use super::material::Material;

#[derive(Default)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, f: f64) -> Self {
        Self {
            albedo,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&r_in.direction().unit_vector(), &rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(),
            r_in.time(),
        );
        *attenuation = self.albedo;

        dot(&scattered.direction(), &rec.normal) > 0.0
    }
}

#[macro_export]
macro_rules! rc_box_metal {
    ( $color:expr, $f:expr ) => {
        Rc::new(Box::new(Metal::new($color, $f)))
    };
    ( $rgb:literal, $f:literal ) => {
        Rc::new(Box::new(Metal::new(Color::with_value($rgb), $f)))
    };
    ( $red:literal, $green:literal, $blue:literal, $f:literal ) => {
        Rc::new(Box::new(Metal::new(
            Color::with_values($red, $green, $blue),
            $f,
        )))
    };
}
