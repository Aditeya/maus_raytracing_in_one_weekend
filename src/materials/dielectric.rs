use rand::{rngs::ThreadRng, Rng};

use crate::{
    hittables::hittable::HitRecord,
    ray::Ray,
    textures::texture::{SolidColor, Texture},
    vec3::{dot, random_in_unit_sphere, random_unit_vector, reflect, refract, Color, Point3},
};

use super::material::Material;

#[derive(Default)]
pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    fn reflectance(cosine: &f64, ref_idx: &f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::with_values(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = 1f64.min(dot(&(-unit_direction), &rec.normal));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Dielectric::reflectance(&cos_theta, &refraction_ratio) > rng.gen::<f64>()
        {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.p, direction, r_in.time());
        true
    }
}

#[macro_export]
macro_rules! rc_box_dielectric {
    ( $ior:expr ) => {
        Rc::new(Box::new(Dielectric::new($ior)))
    };
}
