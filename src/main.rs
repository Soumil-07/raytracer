mod camera;
mod ray;

use std::io;

use camera::Camera;
use indicatif::ProgressBar;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use ray::hittable::{Hittable, HittableList, Sphere};
use ray::utils::write_color;
use ray::{Color, Ray, Vec3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn ray_color(ray: &Ray, world: &impl Hittable, rng: &mut SmallRng, depth: u32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new();
    }

    let (hit, rec) = world.hit(ray, 0.00001, f64::INFINITY);
    if hit {
        let target = rec.point + rec.normal + Vec3::random_in_unit_sphere(rng);
        return ray_color(
            &Ray::new(rec.point, target - rec.point),
            world,
            rng,
            depth - 1,
        ) * 0.5;
    }

    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Color::from_coords(1.0, 1.0, 1.0) * (1.0 - t) + Color::from_coords(0.5, 0.7, 1.0) * t;
}

fn main() {
    // Image dimensions
    let image_width = 400u32;
    let image_height = (image_width as f64 / ASPECT_RATIO) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();
    world.objects.push(Box::new(Sphere::new(
        Vec3::from_coords(0.0, 0.0, -1.0),
        0.5,
    )));
    world.objects.push(Box::new(Sphere::new(
        Vec3::from_coords(0.0, -100.5, -1.0),
        100.0,
    )));

    // camera
    let camera = Camera::new();

    let mut rng = SmallRng::from_entropy();

    println!("P3\n{} {}\n255\n", image_width, image_height);

    // Initialize a progress bar for long renders
    let progress = ProgressBar::new(400 * 225);

    for j in (0..=image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::new();
            // Anti-aliasing
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, &mut rng, max_depth);
            }
            write_color(&mut io::stdout().lock(), pixel_color, samples_per_pixel).unwrap();
            progress.inc(1);
        }
    }

    progress.finish_and_clear();
}
