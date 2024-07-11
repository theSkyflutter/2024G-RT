use crate::{
    color::{self, Color},
    hittable::{HitRecord, Hittable, HittableList},
    interval::Interval,
    ray::Ray,
    rtweekend,
    vec3::{Point3, Vec3},
};
use clap;
use image::{DynamicImage, ImageFormat};
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    f64::INFINITY,
    fs::File,
    sync::{Arc, Mutex},
    thread,
};

#[derive(Clone)]
pub struct Camera {
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    pixel_samples_scale: f64,
    max_depth: u32,
    background: Color,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        background: &Color,
        vfov: f64,
        lookfrom: &Point3,
        lookat: &Point3,
        vup: &Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let image_height = {
            let height = image_width as f64 / aspect_ratio;
            if height < 1.0 {
                1
            } else {
                height as u32
            }
        };

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let center = *lookfrom;

        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * image_width as f64 / image_height as f64;

        let w = (*lookfrom - *lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = center - w * focus_dist - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width,
            image_height,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
            background: *background,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &HittableList) {
        let matches = clap::command!()
            .arg(
                clap::arg!(-o <NAME>)
                    .help("image filename without extension name")
                    .value_parser(clap::value_parser!(String))
                    .default_value("output"),
            )
            .get_matches();
        let path = String::from("output/") + matches.get_one::<String>("NAME").unwrap() + ".png";

        let self_clone = Arc::new(self.clone());
        let world = Arc::new(world.clone());
        let img = Arc::new(Mutex::new(DynamicImage::new_rgb8(
            self.image_width,
            self.image_height,
        )));

        let bar = Arc::new(
            ProgressBar::new((self.image_height * self.image_width) as u64).with_style(
                ProgressStyle::with_template("[{elapsed_precise}] {bar:80} {percent}%").unwrap(),
            ),
        );

        let threads_num = thread::available_parallelism().unwrap().get();
        let mut render_threads = Vec::new();
        for thread_ind in 0..threads_num {
            let self_clone = self_clone.clone();
            let world = world.clone();
            let img = img.clone();
            let bar = bar.clone();
            let render_thread = thread::spawn(move || {
                for j in 0..self_clone.image_height {
                    for i in 0..self_clone.image_width {
                        if (j * self_clone.image_width + i) % threads_num as u32
                            == thread_ind as u32
                        {
                            let mut pixel_color = Color::zeros();
                            for _sample in 0..self_clone.samples_per_pixel {
                                let r = self_clone.get_ray(i, j);
                                pixel_color +=
                                    self_clone.ray_color(&r, self_clone.max_depth, &world);
                            }
                            color::write_color(
                                &(pixel_color * self_clone.pixel_samples_scale),
                                &mut img.lock().unwrap(),
                                i,
                                j,
                            );

                            bar.inc(1);
                        }
                    }
                }
            });
            render_threads.push(render_thread);
        }

        for render_thread in render_threads {
            render_thread.join().unwrap();
        }
        Arc::try_unwrap(bar).unwrap().finish();

        let mut output_file = File::create(&path).unwrap();
        match Arc::try_unwrap(img)
            .unwrap()
            .into_inner()
            .unwrap()
            .write_to(&mut output_file, ImageFormat::Png)
        {
            Ok(_) => {
                println!("Ouput image as \"{}\"", path);
            }
            Err(_) => {
                eprintln!("Outputting image failed");
            }
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64 + offset.x)
                + self.pixel_delta_v * (j as f64 + offset.y));

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = rtweekend::random_double();

        Ray::new_with_time(&ray_origin, &ray_direction, ray_time)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + self.defocus_disk_u * p.x + self.defocus_disk_v * p.y
    }

    fn ray_color(&self, r: &Ray, depth: u32, world: &HittableList) -> Color {
        if depth == 0 {
            Color::zeros()
        } else {
            let mut rec = HitRecord::default();

            if world.hit(r, &Interval::new(0.001, INFINITY), &mut rec) {
                match &rec.mat {
                    Some(mat) => {
                        let mut scattered = Ray::default();
                        let mut attenuation = Color::default();
                        let color_from_emission = mat.emitted(rec.u, rec.v, &rec.p);

                        if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                            let color_from_scatter =
                                attenuation.elemul(&self.ray_color(&scattered, depth - 1, world));
                            color_from_emission + color_from_scatter
                        } else {
                            color_from_emission
                        }
                    }
                    _ => Color::zeros(),
                }
            } else {
                self.background
            }
        }
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(
        rtweekend::random_double() - 0.5,
        rtweekend::random_double() - 0.5,
        0.0,
    )
}
