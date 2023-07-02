use std::sync::Arc;

use crate::vec3::{Color, Point3};

use super::texture::{SolidColor, Texture};

pub struct CheckerTexture {
    odd: Arc<Box<dyn Texture>>,
    even: Arc<Box<dyn Texture>>,
}

impl CheckerTexture {
    pub fn new(odd: &Arc<Box<dyn Texture>>, even: &Arc<Box<dyn Texture>>) -> Self {
        Self {
            odd: Arc::clone(odd),
            even: Arc::clone(even),
        }
    }

    pub fn with_color(odd: Color, even: Color) -> Self {
        Self {
            odd: Arc::new(Box::new(SolidColor::new(odd))),
            even: Arc::new(Box::new(SolidColor::new(even))),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[macro_export]
macro_rules! rc_box_checker_texture {
    ($odd:expr, $even:expr) => {
        Arc::new(Box::new(CheckerTexture::with_color($odd, $even)))
    };
}
