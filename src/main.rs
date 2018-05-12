extern crate raytracing;

extern crate rand;

use std::fs::File;
use std::io::Write;

use rand::{OsRng, Rng};

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
    const NX: u32 = 1600;
    const NY: u32 = 800;
    const NS: u32 = 100;

    let mut gen = OsRng::new().unwrap();

    let sphere1 = Sphere::new(Vec3::new(0., 0., -1.), 0.5);
    let sphere2 = Sphere::new(Vec3::new(0., -100.5, -1.), 100.);

    let world = World::new(vec![Box::new(sphere1), Box::new(sphere2)]);

    let camera = Camera::new();

    let mut file = File::create("test.ppm").unwrap();

    file.write_fmt(format_args!("P3\n{} {}\n255\n", NX, NY)).unwrap();
    for j in (0..NY).rev() {
        for i in 0..NX {
            let mut col = Vec3::new(0., 0., 0.);
            for _ in 0..NS {
                let rand_u: f64 = gen.gen();
                let rand_v: f64 = gen.gen();
                let u = ((i as f64) + rand_u) / (NX as f64);
                let v = ((j as f64) + rand_v) / (NY as f64);

                let r = camera.get_ray(u, v);

                col += colour(r, &world);
            }

            col /= NS as f64;

            let col = Vec3::new(col.e[0].sqrt(), col.e[1].sqrt(), col.e[2].sqrt());
            let ir = (255.99*col.e[0]) as u8;
            let ig = (255.99*col.e[1]) as u8;
            let ib = (255.99*col.e[2]) as u8;

            file.write_fmt(format_args!("{} {} {}\n", ir, ig, ib)).unwrap();
        }
    }
}
