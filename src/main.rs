mod vec3;
use vec3::{Color, Point3, Vec3};

mod ray;
use ray::Ray;

mod hittable;
use hittable::{HitRecord, Hittable, HittableList};

mod sphere;
use sphere::Sphere;

mod camera;
use camera::Camera;

mod material;
use material::{Lambertian, Metal, Material, Dielectric};

use std::io;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::rc::Rc;

use rand::prelude::*;

use clap::Parser;


const ASPECT_RATIO: f32 = 3.0 / 2.0;
const IMAGE_WIDTH: u32 = 1200;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 500;
const MAX_DEPTH: u32 = 50;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Name of the file to output
    #[arg(short, long, value_name = "FILE")]
    filename: String,
}

fn main() {
    let args = Args::parse();

    let filename = format!("{}.ppm", args.filename);
    let file = File::create(filename).expect("Unable to create file");
    let mut file = BufWriter::new(file);

    let ppm_header = format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    write_to_file(&mut file, ppm_header.as_bytes());

    let mut rng = rand::thread_rng();
    let camera = setup_camera();
    let world = random_scene(&mut rng);

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("\rScanlines remaining: {j:03}");
        io::stdout().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT - 1) as f32;
                let ray = camera.get_ray(&mut rng, u, v);
                pixel_color += ray_color(&mut rng, &ray, &world, MAX_DEPTH);
            }
            write_color(&mut file, pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    if file.flush().is_err() {
        eprintln!("Write Failed");
        std::process::exit(1);
    };
    println!("\nDone.");
}

fn write_to_file(file: &mut BufWriter<File>, data_as_bytes: &[u8]) {
    if file.write_all(data_as_bytes).is_err() {
        eprintln!("Write Failed");
        std::process::exit(1);
    }
}

fn write_color(file: &mut BufWriter<File>, pixel_color: Color, samples_per_pixel: u32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f32;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let point = format!(
        "{} {} {}\n",
        (256.0 * r.clamp(0.0, 0.999)) as u32,
        (256.0 * g.clamp(0.0, 0.999)) as u32,
        (256.0 * b.clamp(0.0, 0.999)) as u32
    );

    write_to_file(file, point.as_bytes());
}

fn ray_color(rng: &mut ThreadRng, ray: &Ray, world: &HittableList, depth: u32) -> Color {
    if depth == 0 {
        return Color::new();
    }

    let mut rec = HitRecord::default();

    if world.hit(ray, 0.001, f32::INFINITY, &mut rec) {
        let mut scattered: Ray = Ray::new(&Vec3::new(), &Vec3::new());
        let mut attenutation: Color = Color::new();

        if rec.mat_ptr.scatter(rng, ray, &rec, &mut attenutation, &mut scattered) {
            return attenutation * ray_color(rng, &scattered, world, depth-1);
        }

        return Color::new();
    }

    let unit_direction: Vec3 = ray.direction().unit_vector();
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::with_values(1.0, 1.0, 1.0) + t * Color::with_values(0.5, 0.7, 1.0)
}

fn setup_camera() -> Camera {
    let lookfrom      = Point3::with_values( 13.0, 2.0, 3.0);
    let lookat        = Point3::with_values(  0.0, 0.0, 0.0);
    let vup           =   Vec3::with_values(  0.0, 1.0, 0.0);
    let vertical_fov  = 20.0;
    let aperture      = 0.1;
    let dist_to_focus = 10.0;
    Camera::new(
        lookfrom,
        lookat,
        vup,
        vertical_fov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus
    )
}

fn random_scene(rng: &mut ThreadRng) -> HittableList {
    let mut world = HittableList::new();

    let ground_material: Rc<Box<dyn Material>> = Rc::new(Box::new(Lambertian::new(Color::with_values(0.5, 0.5, 0.5))));
    world.add(Rc::new(Box::new(Sphere::new(Point3::with_values(0.0, -1000.0, 0.0), 1000.0, &ground_material))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            let center = Point3::with_values(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Point3::with_values(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<Box<dyn Material>>;

                if choose_mat < 0.8 {
                    let albedo = Color::random(rng) * Color::random(rng);
                    sphere_material = Rc::new(Box::new(Lambertian::new(albedo)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..=0.5);
                    sphere_material = Rc::new(Box::new(Metal::new(albedo, fuzz)));
                } else {
                    sphere_material = Rc::new(Box::new(Dielectric::new(1.5)));
                }

                world.add(Rc::new(Box::new(Sphere::new(center, 0.2, &sphere_material))));
            }
        }
    }

    let material_1: Rc<Box<dyn Material>> = Rc::new(Box::new(Dielectric::new(1.5)));
    world.add(Rc::new(Box::new(Sphere::new(Point3::with_values(0.0, 1.0, 0.0), 1.0, &material_1))));

    let material_2: Rc<Box<dyn Material>> = Rc::new(Box::new(Lambertian::new(Color::with_values(0.4, 0.2, 0.1))));
    world.add(Rc::new(Box::new(Sphere::new(Point3::with_values(-4.0, 1.0, 0.0), 1.0, &material_2))));

    let material_3: Rc<Box<dyn Material>> = Rc::new(Box::new(Metal::new(Color::with_values(0.7, 0.6, 0.5), 0.0)));
    world.add(Rc::new(Box::new(Sphere::new(Point3::with_values(4.0, 1.0, 0.0), 1.0, &material_3))));

    world
}
