mod aabb;
mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod onb;
mod pdf;
mod ray;
mod rtw_image;
mod rtweekend;
mod texture;
mod vec3;

use crate::{
    camera::Camera,
    color::Color,
    hittable::{Hittable, HittableList, Quad, RotateY, Translate},
    material::{DiffuseLight, Lambertian, Material},
    vec3::{Point3, Vec3},
};
use material::BaseMaterial;
use std::sync::Arc;

fn main() {
    let mut world = HittableList::default();

    let red = Arc::new(Lambertian::from_color(&Color::new(0.65, 0.05, 0.05))) as Arc<dyn Material>;
    let white =
        Arc::new(Lambertian::from_color(&Color::new(0.73, 0.73, 0.73))) as Arc<dyn Material>;
    let green =
        Arc::new(Lambertian::from_color(&Color::new(0.12, 0.45, 0.15))) as Arc<dyn Material>;
    let light =
        Arc::new(DiffuseLight::from_color(&Color::new(15.0, 15.0, 15.0))) as Arc<dyn Material>;

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

    // let mut lights = HittableList::default();
    // let m = Arc::new(BaseMaterial::new()) as Arc<dyn Material>;
    // let lights = Arc::new(Quad::new(
    //     &Point3::new(343.0, 554.0, 332.0),
    //     &Vec3::new(-130.0, 0.0, 0.0),
    //     &Vec3::new(0.0, 0.0, -105.0),
    //     &m,
    // )) as Arc<dyn Hittable>;

    let cam = Camera::new(
        1.0,
        600,
        300,
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
    // cam.render(&world, &lights);
}
