extern crate raytracing;

use std::fs::File;
use std::io::Write;

use raytracing::vec3::Vec3;
use raytracing::ray::Ray;
use raytracing::camera::Camera;
use raytracing::hitable::{World, Hitable};
use raytracing::hitable::surfaces::sphere::Sphere;

fn colour(r: Ray, world: &World) -> Vec3 {
    if let Some(hrec) = world.hit(&r, 0., std::f64::MAX)  {
        let normal = hrec.normal;
        return Vec3::new(normal.e[0] + 1., normal.e[1] + 1., normal.e[2] + 1.)
               * 0.5;
    }

    let unit_direction = r.direction.unit_vector();
    let t = (unit_direction.e[1] + 1.) * 0.5;
    Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t
}

fn main() {
    let nx = 200;
    let ny = 100;


    let sphere1 = Sphere::new(Vec3::new(0., 0., -1.), 0.5);
    let sphere2 = Sphere::new(Vec3::new(0., -100.5, -1.), 100.);

    let world = World::new(vec![Box::new(sphere1), Box::new(sphere2)]);

    let camera = Camera::new();

    let mut file = File::create("test.ppm").unwrap();

    file.write_fmt(format_args!("P3\n{} {}\n255\n", nx, ny)).unwrap();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / (nx as f64);
            let v = j as f64 / (ny as f64);

            let r = camera.get_ray(u, v);


            let col = colour(r, &world);

            let ir = (255.99*col.e[0]) as u8;
            let ig = (255.99*col.e[1]) as u8;
            let ib = (255.99*col.e[2]) as u8;

            file.write_fmt(format_args!("{} {} {}\n", ir, ig, ib)).unwrap();
        }
    }
}
