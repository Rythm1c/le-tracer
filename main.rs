mod src;

use std::io;

use src::color::*;
use src::misc::*;
use src::ray::*;
use src::vec3::*;

use src::camera::Camera;
use src::hittable::{HitRecord, Hittable};
use src::hittable_list::HittableList;
use src::sphere::Sphere;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::ZERO;
    }

    let mut rec = HitRecord::DEFAULT;
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let direction = rec.normal + Vec3::random_unit();
        return 0.5 * ray_color(&Ray::new(rec.p, direction), world, depth - 1);
    }

    let unit_dir = r.dir.unit();
    let t = 0.5 * (unit_dir.y + 1.0);

    (1.0 - t) * Color::ONE + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::new();

    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {j}");

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::ZERO;

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;

                let r = cam.get_ray(u, v);

                pixel_color = pixel_color + ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("\nDone!");
}
