#![allow(dead_code)]
use std::rc::Rc;

use crate::vec3::{Color, Point3};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color_value: Color) -> Self {
        Self { color_value }
    }

    pub fn with_values(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color_value: Color::with_values(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.color_value
    }
}
