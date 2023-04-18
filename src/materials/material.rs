use crate::{
    hittables::hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Point3},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color::with_value(0.0)
    }
}
