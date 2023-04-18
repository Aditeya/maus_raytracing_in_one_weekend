use rand::Rng;
use std::rc::Rc;

use crate::{
    camera::Camera,
    hittables::{
        bvh::BVHNode,
        constant_medium::ConstantMedium,
        cuboid::Cuboid,
        hittable::{Hittable, HittableList},
        moving_sphere::MovingSphere,
        rect::{xy_rect::XYRect, xz_rect::XZRect, yz_rect::YZRect},
        rotate::RotateY,
        sphere::Sphere,
        translate::Translate,
    },
    materials::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian,
        material::Material, metal::Metal,
    },
    rc_box_bvh_node, rc_box_checker_texture, rc_box_constant_medium, rc_box_cuboid,
    rc_box_dielectric, rc_box_diffuse_light, rc_box_image_texture, rc_box_lambertian, rc_box_metal,
    rc_box_moving_sphere, rc_box_noise_texture, rc_box_rotate_y, rc_box_sphere, rc_box_translate,
    rc_box_xy_rect, rc_box_xz_rect, rc_box_yz_rect,
    textures::{
        check_texture::CheckerTexture, image_texture::ImageTexture, perlin::NoiseTexture,
        texture::Texture,
    },
    vec3::{Color, Point3, Vec3},
};

pub struct Settings {
    pub aspect_ratio: f64,
    pub image_width: u64,
    pub image_height: u64,
    pub samples_per_pixel: u64,
    pub max_depth: u64,
}

impl Settings {
    pub fn new() -> Self {
        let aspect_ratio = 3.0 / 2.0;
        let image_width = 1200;
        Self {
            aspect_ratio,
            image_width,
            image_height: (image_width as f64 / aspect_ratio) as u64,
            samples_per_pixel: 500,
            max_depth: 50,
        }
    }

    pub fn set_wdith(&mut self, width: u64) {
        self.image_width = width;
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u64;
    }
}

pub struct Scene;
impl Scene {
    pub fn world_select(world_i: usize) -> (HittableList, Camera, Color, Settings) {
        let (lookfrom, lookat, background);
        let (mut vfov, mut aperture) = (40.0, 0.0);
        let mut settings = Settings::new();

        let world: HittableList;
        match world_i {
            1 => {
                world = Self::random_scene();
                background = Color::with_values(0.70, 0.80, 1.00);
                lookfrom = Point3::with_values(13.0, 2.0, 3.0);
                lookat = Point3::with_value(0.0);

                // settings.samples_per_pixel = 50;
                // settings.set_wdith(200);

                vfov = 20.0;
                aperture = 0.1;
            }
            2 => {
                world = Self::two_sphere();
                background = Color::with_values(0.70, 0.80, 1.00);
                lookfrom = Point3::with_values(13.0, 2.0, 3.0);
                lookat = Point3::with_value(0.0);
                vfov = 20.0;
            }
            3 => {
                world = Self::two_perlin_spheres();
                background = Color::with_values(0.70, 0.80, 1.00);
                lookfrom = Point3::with_values(13.0, 2.0, 3.0);
                lookat = Point3::with_value(0.0);
                vfov = 20.0;
            }
            4 => {
                world = Self::earth();
                background = Color::with_values(0.70, 0.80, 1.00);
                lookfrom = Point3::with_values(13.0, 2.0, 3.0);
                lookat = Point3::with_value(0.0);
                vfov = 20.0;
            }
            5 => {
                world = HittableList::new();
                background = Color::with_value(0.0);
                lookfrom = Point3::with_value(0.0);
                lookat = Point3::with_value(0.0);
            }
            6 => {
                world = Self::simple_light();
                settings.samples_per_pixel = 400;
                background = Color::with_value(0.0);
                lookfrom = Point3::with_values(26.0, 3.0, 6.0);
                lookat = Point3::with_values(0.0, 2.0, 0.0);
                vfov = 20.0;
            }
            7 => {
                world = Self::cornell_box();
                settings.aspect_ratio = 1.0;
                settings.set_wdith(600);
                settings.samples_per_pixel = 400;
                background = Color::with_value(0.0);
                lookfrom = Point3::with_values(278.0, 278.0, -800.0);
                lookat = Point3::with_values(278.0, 278.0, 0.0);
                vfov = 40.0;
            }
            8 => {
                world = Self::cornell_smoke();
                settings.aspect_ratio = 1.0;
                settings.set_wdith(600);
                settings.samples_per_pixel = 200;
                background = Color::with_value(0.0);
                lookfrom = Point3::with_values(278.0, 278.0, -800.0);
                lookat = Point3::with_values(278.0, 278.0, 0.0);
                vfov = 40.0;
            }
            _ => {
                world = Self::final_scene();
                settings.aspect_ratio = 1.0;
                settings.set_wdith(800);
                settings.samples_per_pixel = 10000;
                background = Color::with_value(0.0);
                lookfrom = Point3::with_values(478.0, 278.0, -600.0);
                lookat = Point3::with_values(278.0, 278.0, 0.0);
                vfov = 40.0;
            }
        }

        let camera = Self::setup_camera(lookfrom, lookat, vfov, aperture, &settings);
        (world, camera, background, settings)
    }

    fn setup_camera(
        lookfrom: Point3,
        lookat: Point3,
        vfov: f64,
        aperture: f64,
        settings: &Settings,
    ) -> Camera {
        let vup = Vec3::with_values(0.0, 1.0, 0.0);
        let dist_to_focus = 10.0;
        Camera::new(
            lookfrom,
            lookat,
            vup,
            vfov,
            settings.aspect_ratio,
            aperture,
            dist_to_focus,
            0.0,
            1.0,
        )
    }

    pub fn final_scene() -> HittableList {
        let mut rng = rand::thread_rng();
        let mut boxes1 = HittableList::new();
        let ground: Rc<Box<dyn Material>> = rc_box_lambertian!(0.48, 0.83, 0.53);

        let boxes_per_side = 20;
        for i in 0..boxes_per_side {
            for j in 0..boxes_per_side {
                let w = 100;

                let x0 = -1000.0 + (i * w) as f64;
                let z0 = -1000.0 + (j * w) as f64;
                let y0 = 0.0;

                let x1 = x0 + w as f64;
                let y1 = rng.gen_range(1.0..101.0);
                let z1 = z0 + w as f64;

                boxes1.add(rc_box_cuboid!(
                    Point3::with_values(x0, y0, z0),
                    Point3::with_values(x1, y1, z1),
                    &ground
                ));
            }
        }

        let mut objects = HittableList::new();
        objects.add(rc_box_bvh_node!(&mut boxes1, 0.0, 1.0));

        let light: Rc<Box<dyn Material>> = rc_box_diffuse_light!(7.0);
        objects.add(rc_box_xz_rect!(123.0, 423.0, 147.0, 412.0, 554.0, &light));

        let center1 = Point3::with_values(400.0, 400.0, 200.0);
        let center2 = center1 + Point3::with_values(30.0, 0.0, 0.0);
        let moving_sphere_material: Rc<Box<dyn Material>> = rc_box_lambertian!(0.7, 0.3, 0.1);
        objects.add(rc_box_moving_sphere!(
            center1,
            center2,
            0.0,
            1.0,
            50.0,
            &moving_sphere_material
        ));

        let glass_1p5: Rc<Box<dyn Material>> = rc_box_dielectric!(1.5);

        objects.add(rc_box_sphere!(
            Point3::with_values(260.0, 150.0, 45.0),
            50.0,
            &glass_1p5
        ));
        objects.add(rc_box_sphere!(
            Point3::with_values(0.0, 150.0, 145.0),
            50.0,
            &rc_box_metal!(0.8, 0.8, 0.9, 1.0)
        ));

        let boundary: Rc<Box<dyn Hittable>> =
            rc_box_sphere!(Point3::with_values(360.0, 150.0, 145.0), 70.0, &glass_1p5);
        objects.add(rc_box_constant_medium!(
            &boundary,
            0.2,
            Color::with_values(0.2, 0.4, 0.9),
            Color
        ));
        objects.add(boundary);
        let boundary: Rc<Box<dyn Hittable>> =
            rc_box_sphere!(Point3::with_value(0.0), 5000.0, &glass_1p5);
        objects.add(rc_box_constant_medium!(
            &boundary,
            0.0001,
            Color::with_value(1.0),
            Color
        ));

        let emat: Rc<Box<dyn Material>> =
            rc_box_lambertian!(&rc_box_image_texture!("earthmap.jpg"));
        objects.add(rc_box_sphere!(
            Point3::with_values(400.0, 200.0, 400.0),
            100.0,
            &emat
        ));
        let pertext: Rc<Box<dyn Texture>> = rc_box_noise_texture!(0.1);
        objects.add(rc_box_sphere!(
            Point3::with_values(220.0, 280.0, 300.0),
            80.0,
            &rc_box_lambertian!(&pertext)
        ));

        let mut boxes2 = HittableList::new();
        let white: Rc<Box<dyn Material>> = rc_box_lambertian!(0.73);
        let ns = 1000;
        for j in 0..ns {
            boxes2.add(rc_box_sphere!(
                Point3::random_range(0.0, 165.0),
                10.0,
                &white
            ));
        }

        objects.add(rc_box_translate!(
            &rc_box_rotate_y!(&rc_box_bvh_node!(&mut boxes2, 0.0, 1.0), 15.0),
            Vec3::with_values(-100.0, 270.0, 395.0)
        ));

        objects
    }

    pub fn cornell_smoke() -> HittableList {
        let mut objects = HittableList::new();

        let red: Rc<Box<dyn Material>> = rc_box_lambertian!(0.65, 0.05, 0.05);
        let white: Rc<Box<dyn Material>> = rc_box_lambertian!(0.73);
        let green: Rc<Box<dyn Material>> = rc_box_lambertian!(0.12, 0.45, 0.15);
        let light: Rc<Box<dyn Material>> = rc_box_diffuse_light!(7.0);

        objects.add(rc_box_yz_rect!(000.0, 555.0, 000.0, 555.0, 555.0, &green));
        objects.add(rc_box_yz_rect!(000.0, 555.0, 000.0, 555.0, 000.0, &red));
        objects.add(rc_box_xz_rect!(113.0, 443.0, 127.0, 432.0, 554.0, &light));
        objects.add(rc_box_xz_rect!(000.0, 555.0, 000.0, 555.0, 555.0, &white));
        objects.add(rc_box_xz_rect!(000.0, 555.0, 000.0, 555.0, 000.0, &white));
        objects.add(rc_box_xy_rect!(000.0, 555.0, 000.0, 555.0, 555.0, &white));

        let mut box1: Rc<Box<dyn Hittable>> = rc_box_cuboid!(
            Point3::with_value(0.0),
            Point3::with_values(165.0, 330.0, 165.0),
            &white
        );
        box1 = rc_box_rotate_y!(&box1, 15.0);
        box1 = rc_box_translate!(&box1, Vec3::with_values(265.0, 0.0, 295.0));

        let mut box2: Rc<Box<dyn Hittable>> =
            rc_box_cuboid!(Point3::with_value(0.0), Point3::with_value(165.0), &white);
        box2 = rc_box_rotate_y!(&box2, -18.0);
        box2 = rc_box_translate!(&box2, Vec3::with_values(130.0, 0.0, 65.0));

        objects.add(rc_box_constant_medium!(
            &box1,
            0.01,
            Color::with_value(0.0),
            Color
        ));
        objects.add(rc_box_constant_medium!(
            &box2,
            0.01,
            Color::with_value(1.0),
            Color
        ));

        objects
    }

    pub fn cornell_box() -> HittableList {
        let mut objects = HittableList::new();

        let red: Rc<Box<dyn Material>> = rc_box_lambertian!(0.65, 0.05, 0.05);
        let white: Rc<Box<dyn Material>> = rc_box_lambertian!(0.73);
        let green: Rc<Box<dyn Material>> = rc_box_lambertian!(0.12, 0.45, 0.15);
        let light: Rc<Box<dyn Material>> = rc_box_diffuse_light!(15.0);

        objects.add(rc_box_yz_rect!(000.0, 555.0, 000.0, 555.0, 555.0, &green));
        objects.add(rc_box_yz_rect!(000.0, 555.0, 000.0, 555.0, 000.0, &red));
        objects.add(rc_box_xz_rect!(213.0, 343.0, 227.0, 332.0, 554.0, &light));
        objects.add(rc_box_xz_rect!(000.0, 555.0, 000.0, 555.0, 000.0, &white));
        objects.add(rc_box_xz_rect!(000.0, 555.0, 000.0, 555.0, 555.0, &white));
        objects.add(rc_box_xy_rect!(000.0, 555.0, 000.0, 555.0, 555.0, &white));

        let mut box1: Rc<Box<dyn Hittable>> = rc_box_cuboid!(
            Point3::with_value(0.0),
            Point3::with_values(165.0, 330.0, 165.0),
            &white
        );
        box1 = rc_box_rotate_y!(&box1, 15.0);
        box1 = rc_box_translate!(&box1, Vec3::with_values(265.0, 0.0, 295.0));
        objects.add(box1);

        let mut box2: Rc<Box<dyn Hittable>> =
            rc_box_cuboid!(Point3::with_value(0.0), Point3::with_value(165.0), &white);
        box2 = rc_box_rotate_y!(&box2, -18.0);
        box2 = rc_box_translate!(&box2, Vec3::with_values(130.0, 0.0, 65.0));
        objects.add(box2);

        objects
    }

    pub fn simple_light() -> HittableList {
        let mut objects = HittableList::new();

        let perlin_texture: Rc<Box<dyn Texture>> = rc_box_noise_texture!(4.0);
        let perlin: Rc<Box<dyn Material>> = rc_box_lambertian!(&perlin_texture);

        let sphere1: Rc<Box<dyn Hittable>> =
            rc_box_sphere!(Point3::with_values(0.0, -1000.0, 0.0), 1000.0, &perlin);
        let sphere2: Rc<Box<dyn Hittable>> =
            rc_box_sphere!(Point3::with_values(0.0, 2.0, 0.0), 2.0, &perlin);
        objects.add(sphere1);
        objects.add(sphere2);

        let difflight: Rc<Box<dyn Material>> = rc_box_diffuse_light!(4.0);
        let xy_rect: Rc<Box<dyn Hittable>> = rc_box_xy_rect!(3.0, 5.0, 1.0, 3.0, -2.0, &difflight);
        objects.add(xy_rect);

        objects
    }

    pub fn earth() -> HittableList {
        let mut objects = HittableList::new();

        let earth_texture: Rc<Box<dyn Texture>> = rc_box_image_texture!("earthmap.jpg");
        let earth_material: Rc<Box<dyn Material>> = rc_box_lambertian!(&earth_texture);

        let sphere: Rc<Box<dyn Hittable>> =
            rc_box_sphere!(Point3::with_value(0.0), 2.0, &earth_material);
        objects.add(sphere);
        objects
    }

    pub fn two_perlin_spheres() -> HittableList {
        let mut objects = HittableList::new();

        let perlin_texture: Rc<Box<dyn Texture>> = rc_box_noise_texture!(4.0);
        let perlin: Rc<Box<dyn Material>> = rc_box_lambertian!(&perlin_texture);

        let sphere1: Rc<Box<dyn Hittable>> =
            rc_box_sphere!(Point3::with_values(0.0, -1000.0, 0.0), 1000.0, &perlin);
        let sphere2: Rc<Box<dyn Hittable>> =
            rc_box_sphere!(Point3::with_values(0.0, 2.0, 0.0), 2.0, &perlin);
        objects.add(sphere1);
        objects.add(sphere2);

        objects
    }

    pub fn two_sphere() -> HittableList {
        let mut objects = HittableList::new();

        let odd = Color::with_values(0.2, 0.3, 0.1);
        let even = Color::with_values(0.9, 0.9, 0.9);
        let checker_texture: Rc<Box<dyn Texture>> = rc_box_checker_texture!(odd, even);
        let checker: Rc<Box<dyn Material>> = rc_box_lambertian!(&checker_texture);

        let sphere1: Rc<Box<dyn Hittable>> =
            rc_box_sphere!(Point3::with_values(0.0, -10.0, 0.0), 10.0, &checker);
        let sphere2: Rc<Box<dyn Hittable>> =
            rc_box_sphere!(Point3::with_values(0.0, 10.0, 0.0), 10.0, &checker);
        objects.add(sphere1);
        objects.add(sphere2);

        objects
    }

    pub fn random_scene() -> HittableList {
        let mut rng = rand::thread_rng();
        let mut world = HittableList::new();

        let odd = Color::with_values(0.2, 0.3, 0.1);
        let even = Color::with_values(0.9, 0.9, 0.9);
        let checker_texture: Rc<Box<dyn Texture>> = rc_box_checker_texture!(odd, even);
        let ground_material: Rc<Box<dyn Material>> = rc_box_lambertian!(&checker_texture);
        world.add(rc_box_sphere!(
            Point3::with_values(0.0, -1000.0, 0.0),
            1000.0,
            &ground_material
        ));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat: f64 = rng.gen();
                let center = Point3::with_values(
                    a as f64 + 0.9 * rng.gen::<f64>(),
                    0.2,
                    b as f64 + 0.9 * rng.gen::<f64>(),
                );

                if (center - Point3::with_values(4.0, 0.2, 0.0)).length() > 0.9 {
                    let sphere_material: Rc<Box<dyn Material>>;

                    if choose_mat < 0.8 {
                        // NOTE: diffuse
                        let albedo = Color::random() * Color::random();
                        sphere_material = rc_box_lambertian!(Color, albedo);

                        let center2 =
                            center + Vec3::with_values(0.0, rng.gen_range(0.0..=0.5), 0.0);
                        world.add(rc_box_moving_sphere!(
                            center,
                            center2,
                            0.0,
                            1.0,
                            0.2,
                            &sphere_material
                        ));
                        continue;
                    } else if choose_mat < 0.95 {
                        // NOTE: metal
                        let albedo = Color::random_range(0.5, 1.0);
                        let fuzz = rng.gen_range(0.0..=0.5);
                        sphere_material = rc_box_metal!(albedo, fuzz);
                    } else {
                        // NOTE: glass
                        sphere_material = rc_box_dielectric!(1.5);
                    }

                    world.add(rc_box_sphere!(center, 0.2, &sphere_material));
                }
            }
        }

        let material_1: Rc<Box<dyn Material>> = rc_box_dielectric!(1.5);
        world.add(rc_box_sphere!(
            Point3::with_values(0.0, 1.0, 0.0),
            1.0,
            &material_1
        ));

        let material_2: Rc<Box<dyn Material>> = rc_box_lambertian!(0.4, 0.2, 0.1);
        world.add(rc_box_sphere!(
            Point3::with_values(-4.0, 1.0, 0.0),
            1.0,
            &material_2
        ));

        let material_3: Rc<Box<dyn Material>> = rc_box_metal!(0.7, 0.6, 0.5, 0.0);
        world.add(rc_box_sphere!(
            Point3::with_values(4.0, 1.0, 0.0),
            1.0,
            &material_3
        ));

        world
    }
}
