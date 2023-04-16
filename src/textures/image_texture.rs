use std::path::Path;

use image::io::Reader;
use image::RgbaImage;

use crate::vec3::{Color, Point3};

use super::texture::Texture;

pub struct ImageTexture {
    data: Option<RgbaImage>,
}

impl ImageTexture {
    pub fn new<T: AsRef<Path>>(file: T) -> Self {
        let data = Reader::open(file)
            .ok()
            .and_then(|x| x.decode().map(|x| x.to_rgba8()).ok());
        Self { data }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        if let Some(data) = &self.data {
            let u = u.clamp(0.0, 1.0);
            let v = 1.0 - v.clamp(0.0, 1.0);

            let (i, j) = {
                let width = data.width() as u64;
                let height = data.height() as u64;
                let mut i = (u * width as f64) as u64;
                let mut j = (v * height as f64) as u64;

                if i > width {
                    i = width - 1;
                }
                if j > height {
                    j = height - 1;
                }
                (i, j)
            };

            let color_scale = 1.0 / 255.0;
            let pixel = data.get_pixel(i as u32, j as u32).0;
            Color::with_values(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64) * color_scale
        } else {
            Color::with_values(0.0, 1.0, 1.0)
        }
    }
}

#[macro_export]
macro_rules! rc_box_image_texture {
    ($filepath:expr) => {
        Rc::new(Box::new(ImageTexture::new($filepath)))
    };
}
