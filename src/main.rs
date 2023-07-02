#![allow(dead_code, unused)]
mod scene;
mod vec3;

mod hittables;
mod materials;
mod textures;

mod aabb;
mod camera;
mod ray;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;

use hittables::hittable::{HitRecord, Hittable, HittableList};
use ray::Ray;
use scene::Scene;
use vec3::{Color, Vec3};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Scene to render
    #[arg(short, long, value_name = "NUM", default_value_t = 0)]
    scene_number: usize,

    // Name of the file to output
    #[arg(short, long, value_name = "FILE", default_value_t = String::from("image"))]
    filename: String,
}

fn main() {
    let args = Args::parse();

    let (world, camera, background, settings) = Scene::world_select(args.scene_number);
    let world = Arc::new(world);

    let filename = format!("{}.ppm", args.filename);
    let file = File::create(filename).expect("Unable to create file");
    let mut file = BufWriter::new(file);

    let ppm_header = format!(
        "P3\n{} {}\n255\n",
        settings.image_width, settings.image_height
    );
    write_to_file(&mut file, ppm_header.as_bytes());

    let progress_bar = ProgressBar::new(settings.image_height);
    progress_bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.red} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>4}/{len:4} {msg}",
        )
        .unwrap(),
    );
    progress_bar.set_message("WORK");
    let progress_bar = Mutex::new(progress_bar);

    let mut img: Arc<Mutex<Vec<Vec<[u8; 12]>>>> =
        Arc::new(Mutex::new(vec![
            vec![
                [48u8; 12];
                settings.image_width as usize
            ];
            settings.image_height as usize
        ]));

    (0..settings.image_height).into_par_iter().for_each(|j| {
        progress_bar.lock().unwrap().inc(1);
        let buf_j = (settings.image_height - 1 - j) as usize;

        (0..settings.image_width).into_par_iter().for_each(|i| {
            let pixel_color = (0..settings.samples_per_pixel)
                .into_par_iter()
                .map(|_| {
                    let mut rng = rand::thread_rng();
                    let u = (i as f64 + rng.gen::<f64>()) / (settings.image_width - 1) as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / (settings.image_height - 1) as f64;
                    let ray = camera.get_ray(u, v);
                    ray_color(ray, &background, &world, settings.max_depth)
                })
                .sum();

            let mut img = img.lock().unwrap();
            write_to_buf(
                &mut img,
                buf_j,
                i as usize,
                pixel_color,
                settings.samples_per_pixel,
            );
        })
    });

    let img = img.lock().unwrap().concat().concat();
    write_to_file(&mut file, &img);

    if let Err(e) = file.flush() {
        eprintln!("Write Failed because: {}", e);
        std::process::exit(1);
    };

    progress_bar.lock().unwrap().finish_with_message("DONE");
}

fn write_to_buf(
    buf: &mut [Vec<[u8; 12]>],
    j: usize,
    i: usize,
    pixel_color: Color,
    samples_per_pixel: u64,
) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let point = format!(
        "{:#03} {:#03} {:#03}\n",
        (256.0 * r.clamp(0.0, 0.999)) as u64,
        (256.0 * g.clamp(0.0, 0.999)) as u64,
        (256.0 * b.clamp(0.0, 0.999)) as u64
    );

    buf[j][i].copy_from_slice(point.as_bytes());
}

fn write_to_file(file: &mut BufWriter<File>, data_as_bytes: &[u8]) {
    if file.write(data_as_bytes).is_err() {
        eprintln!("Write Failed");
        std::process::exit(1);
    }
}

fn write_color(file: &mut BufWriter<File>, pixel_color: Color, samples_per_pixel: u64) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let point = format!(
        "{:#03} {:#03} {:#03}\n",
        (256.0 * r.clamp(0.0, 0.999)) as u64,
        (256.0 * g.clamp(0.0, 0.999)) as u64,
        (256.0 * b.clamp(0.0, 0.999)) as u64
    );

    write_to_file(file, point.as_bytes());
}

fn ray_color_recursive(ray: &Ray, background: &Color, world: &HittableList, depth: u64) -> Color {
    if depth == 0 {
        return Color::with_value(0.0);
    }

    let mut rec = HitRecord::default();

    if !world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
        return *background;
    }

    let mut scattered: Ray = Ray::new(Vec3::new(), Vec3::new(), 0.0);
    let mut attenutation: Color = Color::new();
    let emitted: Color = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);

    if !rec
        .mat_ptr
        .scatter(ray, &rec, &mut attenutation, &mut scattered)
    {
        return emitted;
    }

    emitted + attenutation * ray_color_recursive(&scattered, background, world, depth - 1)
}

fn ray_color(mut ray: Ray, background: &Color, world: &HittableList, depth: u64) -> Color {
    let mut emitted_attenuation: Vec<(Color, Color)> = Vec::with_capacity(depth as usize);

    let mut final_ray_color: Color = Color::with_value(0.0);
    for _ in (0..depth).rev() {
        let mut rec = HitRecord::default();

        if !world.hit(&ray, 0.001, f64::INFINITY, &mut rec) {
            final_ray_color = *background;
            break;
        }

        let mut scattered: Ray = Ray::new(Vec3::new(), Vec3::new(), 0.0);
        let mut attenutation: Color = Color::new();
        let emitted: Color = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);

        if !rec
            .mat_ptr
            .scatter(&ray, &rec, &mut attenutation, &mut scattered)
        {
            final_ray_color = emitted;
            break;
        }

        emitted_attenuation.push((emitted, attenutation));
        ray = scattered;
    }

    emitted_attenuation
        .iter()
        .rev()
        .fold(final_ray_color, |mut acc, &(emitted, attenutation)| {
            acc = emitted + attenutation * acc;
            acc
        })
}
