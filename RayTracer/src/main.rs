mod aabb;
mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod rtw_image;
mod rtweekend;
mod texture;
mod vec3;

use crate::{
    camera::Camera,
    color::Color,
    hittable::{BvhNode, ConstantMedium, Hittable, HittableList, Quad, RotateY, Sphere, Translate},
    material::{Dielectric, DiffuseLight, Lambertian, Material, Metal},
    texture::{CheckerTexture, ImageTexture, NoiseTexture, Texture},
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

fn main() {
    match 9 {
        1 => {
            bouncing_spheres();
        }
        2 => {
            checkered_spheres();
        }
        3 => {
            earth();
        }
        4 => {
            perlin_spheres();
        }
        5 => {
            quads();
        }
        6 => {
            simple_light();
        }
        7 => {
            cornell_box();
        }
        8 => {
            cornell_smoke();
        }
        9 => {
            final_scene(800, 10000, 40);
        }
        _ => {
            final_scene(400, 250, 4);
        }
    }
}

fn bouncing_spheres() {
    // World
    let mut world = HittableList::default();

    let ground_material: Arc<dyn Material> =
        Arc::new(Lambertian::from_color(&Color::new(0.5, 0.5, 0.5)));
    world.add(
        &(Arc::new(Sphere::new(
            &Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            &ground_material,
        )) as Arc<dyn Hittable>),
    );

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rtweekend::random_double();
            let center = Point3::new(
                a as f64 + 0.9 * rtweekend::random_double(),
                0.2,
                b as f64 + rtweekend::random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0).length()).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo = Color::random().elemul(&Color::random());
                    sphere_material = Arc::new(Lambertian::from_color(&albedo));
                    let center2 =
                        center + Vec3::new(0.0, rtweekend::random_double_in_range(0.0, 0.5), 0.0);
                    world.add(
                        &(Arc::new(Sphere::new_moving(&center, &center2, 0.2, &sphere_material))
                            as Arc<dyn Hittable>),
                    );
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_in_range(0.5, 1.0);
                    let fuzz = rtweekend::random_double_in_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(&albedo, fuzz));
                    world.add(
                        &(Arc::new(Sphere::new(&center, 0.2, &sphere_material))
                            as Arc<dyn Hittable>),
                    );
                } else {
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(
                        &(Arc::new(Sphere::new(&center, 0.2, &sphere_material))
                            as Arc<dyn Hittable>),
                    );
                }
            }
        }
    }

    let material1: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    world.add(
        &(Arc::new(Sphere::new(&Point3::new(0.0, 1.0, 0.0), 1.0, &material1)) as Arc<dyn Hittable>),
    );

    let material2: Arc<dyn Material> = Arc::new(Lambertian::from_color(&Color::new(0.4, 0.2, 0.1)));
    world.add(
        &(Arc::new(Sphere::new(&Point3::new(-4.0, 1.0, 0.0), 1.0, &material2))
            as Arc<dyn Hittable>),
    );

    let material3: Arc<dyn Material> = Arc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(
        &(Arc::new(Sphere::new(&Point3::new(4.0, 1.0, 0.0), 1.0, &material3)) as Arc<dyn Hittable>),
    );

    world = HittableList::new(
        &(Arc::new(BvhNode::from_hittable_list(&mut world)) as Arc<dyn Hittable>),
    );

    let cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        20,
        &Color::new(0.7, 0.8, 1.0),
        20.0,
        &Point3::new(13.0, 2.0, 3.0),
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );
    cam.render(&world);
}

fn checkered_spheres() {
    let mut world = HittableList::default();

    let checker: Arc<dyn Texture> = Arc::new(CheckerTexture::from_colors(
        0.32,
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    ));

    world.add(
        &(Arc::new(Sphere::new(
            &Point3::new(0.0, -10.0, 0.0),
            10.0,
            &(Arc::new(Lambertian::new(&checker)) as Arc<dyn Material>),
        )) as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(Sphere::new(
            &Point3::new(0.0, 10.0, 0.0),
            10.0,
            &(Arc::new(Lambertian::new(&checker)) as Arc<dyn Material>),
        )) as Arc<dyn Hittable>),
    );

    let cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        &Color::new(0.7, 0.8, 1.0),
        20.0,
        &Point3::new(13.0, 2.0, 3.0),
        &Point3::zeros(),
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );
    cam.render(&world);
}

fn earth() {
    let earth_texture: Arc<dyn Texture> = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface: Arc<dyn Material> = Arc::new(Lambertian::new(&earth_texture));
    let globe = Arc::new(Sphere::new(
        &Point3::new(0.0, 0.0, 0.0),
        2.0,
        &earth_surface,
    ));

    let cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        &Color::new(0.7, 0.8, 1.0),
        20.0,
        &Point3::new(13.0, 2.0, 3.0),
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );
    cam.render(&HittableList::new(&(globe as Arc<dyn Hittable>)));
}

fn perlin_spheres() {
    let mut world = HittableList::default();

    let pertext: Arc<dyn Texture> = Arc::new(NoiseTexture::new(4.0));
    world.add(
        &(Arc::new(Sphere::new(
            &Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            &(Arc::new(Lambertian::new(&pertext)) as Arc<dyn Material>),
        )) as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(Sphere::new(
            &Point3::new(0.0, 2.0, 0.0),
            2.0,
            &(Arc::new(Lambertian::new(&pertext)) as Arc<dyn Material>),
        )) as Arc<dyn Hittable>),
    );

    let cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        &Color::new(0.7, 0.8, 1.0),
        20.0,
        &Point3::new(13.0, 2.0, 3.0),
        &Point3::zeros(),
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );
    cam.render(&world);
}

fn quads() {
    let mut world = HittableList::default();

    let left_red: Arc<dyn Material> = Arc::new(Lambertian::from_color(&Color::new(1.0, 0.2, 0.2)));
    let back_green: Arc<dyn Material> =
        Arc::new(Lambertian::from_color(&Color::new(0.2, 1.0, 0.2)));
    let right_blue: Arc<dyn Material> =
        Arc::new(Lambertian::from_color(&Color::new(0.2, 0.2, 1.0)));
    let upper_orange: Arc<dyn Material> =
        Arc::new(Lambertian::from_color(&Color::new(1.0, 0.5, 0.0)));
    let lower_teal: Arc<dyn Material> =
        Arc::new(Lambertian::from_color(&Color::new(0.2, 0.8, 0.8)));

    world.add(
        &(Arc::new(Quad::new(
            &Point3::new(-3.0, -2.0, 5.0),
            &Vec3::new(0.0, 0.0, -4.0),
            &Vec3::new(0.0, 4.0, 0.0),
            &left_red,
        )) as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(Quad::new(
            &Point3::new(-2.0, -2.0, 0.0),
            &Vec3::new(4.0, 0.0, 0.0),
            &Vec3::new(0.0, 4.0, 0.0),
            &back_green,
        )) as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(Quad::new(
            &Point3::new(3.0, -2.0, 1.0),
            &Vec3::new(0.0, 0.0, 4.0),
            &Vec3::new(0.0, 4.0, 0.0),
            &right_blue,
        )) as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(Quad::new(
            &Point3::new(-2.0, 3.0, 1.0),
            &Vec3::new(4.0, 0.0, 0.0),
            &Vec3::new(0.0, 0.0, 4.0),
            &upper_orange,
        )) as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(Quad::new(
            &Point3::new(-2.0, -3.0, 5.0),
            &Vec3::new(4.0, 0.0, 0.0),
            &Vec3::new(0.0, 0.0, -4.0),
            &lower_teal,
        )) as Arc<dyn Hittable>),
    );

    let cam = Camera::new(
        1.0,
        400,
        100,
        50,
        &Color::new(0.7, 0.8, 1.0),
        80.0,
        &Point3::new(0.0, 0.0, 9.0),
        &Point3::zeros(),
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );
    cam.render(&world);
}

fn simple_light() {
    let mut world = HittableList::default();

    let pertext: Arc<dyn Texture> = Arc::new(NoiseTexture::new(4.0));
    world.add(
        &(Arc::new(Sphere::new(
            &Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            &(Arc::new(Lambertian::new(&pertext)) as Arc<dyn Material>),
        )) as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(Sphere::new(
            &Point3::new(0.0, 2.0, 0.0),
            2.0,
            &(Arc::new(Lambertian::new(&pertext)) as Arc<dyn Material>),
        )) as Arc<dyn Hittable>),
    );

    let difflight: Arc<dyn Material> =
        Arc::new(DiffuseLight::from_color(&Color::new(4.0, 4.0, 4.0)));
    world.add(
        &(Arc::new(Sphere::new(&Point3::new(0.0, 7.0, 0.0), 2.0, &difflight)) as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(Quad::new(
            &Point3::new(3.0, 1.0, -2.0),
            &Vec3::new(2.0, 0.0, 0.0),
            &Vec3::new(0.0, 2.0, 0.0),
            &difflight,
        )) as Arc<dyn Hittable>),
    );

    let cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        &Color::zeros(),
        20.0,
        &Point3::new(26.0, 3.0, 6.0),
        &Point3::new(0.0, 2.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );
    cam.render(&world);
}

fn cornell_box() {
    let mut world = HittableList::default();

    let red: Arc<dyn Material> = Arc::new(Lambertian::from_color(&Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::from_color(&Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::from_color(&Color::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material> =
        Arc::new(DiffuseLight::from_color(&Color::new(15.0, 15.0, 15.0)));

    world.add(
        &(Arc::new(Quad::new(
            &Point3::new(555.0, 0.0, 0.0),
            &Vec3::new(0.0, 555.0, 0.0),
            &Vec3::new(0.0, 0.0, 555.0),
            &green,
        )) as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(Quad::new(
            &Point3::new(0.0, 0.0, 0.0),
            &Vec3::new(0.0, 555.0, 0.0),
            &Vec3::new(0.0, 0.0, 555.0),
            &red,
        )) as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(Quad::new(
            &Point3::new(343.0, 554.0, 332.0),
            &Vec3::new(-130.0, 0.0, 0.0),
            &Vec3::new(0.0, 0.0, -105.0),
            &light,
        )) as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(Quad::new(
            &Point3::new(0.0, 0.0, 0.0),
            &Vec3::new(555.0, 0.0, 0.0),
            &Vec3::new(0.0, 0.0, 555.0),
            &white,
        )) as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(Quad::new(
            &Point3::new(555.0, 555.0, 555.0),
            &Vec3::new(-555.0, 0.0, 0.0),
            &Vec3::new(0.0, 0.0, -555.0),
            &white,
        )) as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(Quad::new(
            &Point3::new(0.0, 0.0, 555.0),
            &Vec3::new(555.0, 0.0, 0.0),
            &Vec3::new(0.0, 555.0, 0.0),
            &white,
        )) as Arc<dyn Hittable>),
    );

    let mut box1: Arc<dyn Hittable> =
        hittable::get_box(&Point3::zeros(), &Point3::new(165.0, 330.0, 165.0), &white);
    box1 = Arc::new(RotateY::new(&box1, 15.0));
    box1 = Arc::new(Translate::new(&box1, &Vec3::new(265.0, 0.0, 295.0)));
    world.add(&box1);

    let mut box2: Arc<dyn Hittable> =
        hittable::get_box(&Point3::zeros(), &Point3::new(165.0, 165.0, 165.0), &white);
    box2 = Arc::new(RotateY::new(&box2, -18.0));
    box2 = Arc::new(Translate::new(&box2, &Vec3::new(130.0, 0.0, 65.0)));
    world.add(&box2);

    let cam = Camera::new(
        1.0,
        600,
        200,
        50,
        &Color::zeros(),
        40.0,
        &Point3::new(278.0, 278.0, -800.0),
        &Point3::new(278.0, 278.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );
    cam.render(&world);
}

fn cornell_smoke() {
    let mut world = HittableList::default();

    let red: Arc<dyn Material> = Arc::new(Lambertian::from_color(&Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::from_color(&Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::from_color(&Color::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material> = Arc::new(DiffuseLight::from_color(&Color::new(7.0, 7.0, 7.0)));

    world.add(
        &(Arc::new(Quad::new(
            &Point3::new(555.0, 0.0, 0.0),
            &Vec3::new(0.0, 555.0, 0.0),
            &Vec3::new(0.0, 0.0, 555.0),
            &green,
        )) as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(Quad::new(
            &Point3::new(0.0, 0.0, 0.0),
            &Vec3::new(0.0, 555.0, 0.0),
            &Vec3::new(0.0, 0.0, 555.0),
            &red,
        )) as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(Quad::new(
            &Point3::new(113.0, 554.0, 127.0),
            &Vec3::new(330.0, 0.0, 0.0),
            &Vec3::new(0.0, 0.0, 305.0),
            &light,
        )) as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(Quad::new(
            &Point3::new(0.0, 555.0, 0.0),
            &Vec3::new(555.0, 0.0, 0.0),
            &Vec3::new(0.0, 0.0, 555.0),
            &white,
        )) as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(Quad::new(
            &Point3::new(0.0, 0.0, 0.0),
            &Vec3::new(555.0, 0.0, 0.0),
            &Vec3::new(0.0, 0.0, 555.0),
            &white,
        )) as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(Quad::new(
            &Point3::new(0.0, 0.0, 555.0),
            &Vec3::new(555.0, 0.0, 0.0),
            &Vec3::new(0.0, 555.0, 0.0),
            &white,
        )) as Arc<dyn Hittable>),
    );

    let mut box1: Arc<dyn Hittable> =
        hittable::get_box(&Point3::zeros(), &Point3::new(165.0, 330.0, 165.0), &white);
    box1 = Arc::new(RotateY::new(&box1, 15.0));
    box1 = Arc::new(Translate::new(&box1, &Vec3::new(265.0, 0.0, 295.0)));

    let mut box2: Arc<dyn Hittable> =
        hittable::get_box(&Point3::zeros(), &Point3::new(165.0, 165.0, 165.0), &white);
    box2 = Arc::new(RotateY::new(&box2, -18.0));
    box2 = Arc::new(Translate::new(&box2, &Vec3::new(130.0, 0.0, 65.0)));

    world.add(
        &(Arc::new(ConstantMedium::new_with_color(&box1, 0.01, &Color::zeros()))
            as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(ConstantMedium::new_with_color(&box2, 0.01, &Color::ones()))
            as Arc<dyn Hittable>),
    );

    let cam = Camera::new(
        1.0,
        600,
        200,
        50,
        &Color::zeros(),
        40.0,
        &Point3::new(278.0, 278.0, -800.0),
        &Point3::new(278.0, 278.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );
    cam.render(&world);
}

fn final_scene(image_width: u32, samples_per_pixel: u32, max_depth: u32) {
    let mut boxes1 = HittableList::default();
    let ground: Arc<dyn Material> = Arc::new(Lambertian::from_color(&Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rtweekend::random_double_in_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(
                &(hittable::get_box(&Point3::new(x0, y0, z0), &Point3::new(x1, y1, z1), &ground)
                    as Arc<dyn Hittable>),
            );
        }
    }

    let mut world = HittableList::default();

    world.add(&(Arc::new(BvhNode::from_hittable_list(&mut boxes1)) as Arc<dyn Hittable>));

    let light: Arc<dyn Material> = Arc::new(DiffuseLight::from_color(&Color::new(7.0, 7.0, 7.0)));
    world.add(
        &(Arc::new(Quad::new(
            &Point3::new(123.0, 554.0, 147.0),
            &Vec3::new(300.0, 0.0, 0.0),
            &Vec3::new(0.0, 0.0, 265.0),
            &light,
        )) as Arc<dyn Hittable>),
    );

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material: Arc<dyn Material> =
        Arc::new(Lambertian::from_color(&Color::new(0.7, 0.3, 0.1)));
    world.add(
        &(Arc::new(Sphere::new_moving(
            &center1,
            &center2,
            50.0,
            &sphere_material,
        )) as Arc<dyn Hittable>),
    );

    world.add(
        &(Arc::new(Sphere::new(
            &Point3::new(260.0, 150.0, 45.0),
            50.0,
            &(Arc::new(Dielectric::new(1.5)) as Arc<dyn Material>),
        )) as Arc<dyn Hittable>),
    );
    world.add(
        &(Arc::new(Sphere::new(
            &Point3::new(0.0, 150.0, 145.0),
            50.0,
            &(Arc::new(Metal::new(&Color::new(0.8, 0.8, 0.9), 1.0)) as Arc<dyn Material>),
        )) as Arc<dyn Hittable>),
    );

    let mut boundary: Arc<dyn Hittable> = Arc::new(Sphere::new(
        &Point3::new(260.0, 150.0, 45.0),
        50.0,
        &(Arc::new(Dielectric::new(1.5)) as Arc<dyn Material>),
    ));
    world.add(&boundary);
    world.add(
        &(Arc::new(ConstantMedium::new_with_color(
            &boundary,
            0.2,
            &Color::new(0.2, 0.4, 0.9),
        )) as Arc<dyn Hittable>),
    );
    boundary = Arc::new(Sphere::new(
        &Point3::zeros(),
        5000.0,
        &(Arc::new(Dielectric::new(1.5)) as Arc<dyn Material>),
    ));
    world.add(
        &(Arc::new(ConstantMedium::new_with_color(
            &boundary,
            0.0001,
            &Color::ones(),
        )) as Arc<dyn Hittable>),
    );

    let emat: Arc<dyn Material> = Arc::new(Lambertian::new(
        &(Arc::new(ImageTexture::new("earthmap.jpg")) as Arc<dyn Texture>),
    ));
    world.add(
        &(Arc::new(Sphere::new(&Point3::new(400.0, 200.0, 400.0), 100.0, &emat))
            as Arc<dyn Hittable>),
    );
    let pertext: Arc<dyn Texture> = Arc::new(NoiseTexture::new(0.2));
    world.add(
        &(Arc::new(Sphere::new(
            &Point3::new(220.0, 280.0, 300.0),
            80.0,
            &(Arc::new(Lambertian::new(&pertext)) as Arc<dyn Material>),
        )) as Arc<dyn Hittable>),
    );

    let mut boxes2 = HittableList::default();
    let white: Arc<dyn Material> = Arc::new(Lambertian::from_color(&Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(
            &(Arc::new(Sphere::new(
                &Point3::random_in_range(0.0, 165.0),
                10.0,
                &white,
            )) as Arc<dyn Hittable>),
        );
    }

    world.add(
        &(Arc::new(Translate::new(
            &(Arc::new(RotateY::new(
                &(Arc::new(BvhNode::from_hittable_list(&mut boxes2)) as Arc<dyn Hittable>),
                15.0,
            )) as Arc<dyn Hittable>),
            &Vec3::new(-100.0, 270.0, 395.0),
        )) as Arc<dyn Hittable>),
    );

    let cam = Camera::new(
        1.0,
        image_width,
        samples_per_pixel,
        max_depth,
        &Color::zeros(),
        40.0,
        &Point3::new(478.0, 278.0, -600.0),
        &Point3::new(278.0, 278.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );
    cam.render(&world);
}
