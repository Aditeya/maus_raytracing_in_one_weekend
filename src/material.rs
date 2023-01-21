use rand::prelude::*;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{random_unit_vector, Color, reflect, dot, random_in_unit_sphere, refract},
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
}

#[derive(Default)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector(rng);

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(&rec.p, &scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Default)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, f: f32) -> Self {
        Self {
            albedo,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&r_in.direction().unit_vector(), &rec.normal);
        *scattered = Ray::new(&rec.p, &(reflected + self.fuzz*random_in_unit_sphere(rng)));
        *attenuation = self.albedo;

        dot(&scattered.direction(), &rec.normal) > 0.0
    }
}


#[derive(Default)]
pub struct Dielectric {
    pub ir: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Self {
        Self { ir: index_of_refraction }
    }

    fn reflectance(cosine: &f32, ref_idx: &f32) -> f32 {
        let mut r0 = (1.0-ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0)*(1.0 - cosine).powi(5)
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
            1.0/self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = 1f32.min(dot(&(-unit_direction), &rec.normal));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || Dielectric::reflectance(&cos_theta, &refraction_ratio) > rng.gen::<f32>() {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };


        *scattered = Ray::new(&rec.p, &direction);
        true
    }
}
