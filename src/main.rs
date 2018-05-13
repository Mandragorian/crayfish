extern crate raytracing;

extern crate rand;

use std::fs::File;
use std::io::Write;
use std::f64::consts::PI;

use rand::{OsRng, Rng};

use raytracing::vec3::Vec3;
use raytracing::ray::Ray;
use raytracing::camera::Camera;
use raytracing::hitable::{World, Hitable};
use raytracing::hitable::surfaces::sphere::Sphere;
use raytracing::material::common::lambertian::Lambertian;
use raytracing::material::common::metal::Metal;
use raytracing::material::common::dielectric::Dielectric;

fn colour<'a, T>(r: Ray, world: &T, depth: usize) -> Vec3
where
    T: Hitable<'a>
{
    if let Some(hrec) = world.hit(&r, 0.0001, std::f64::MAX)  {
        if depth < 50 {
            if let Some((att, scat)) = hrec.material.scatter(&r, &hrec) {
                return att * colour(scat, world, depth + 1);
            }
        }
        return Vec3::new(0., 0., 0.);
    }
    let unit_direction = r.direction.unit_vector();
    let t = (unit_direction.e[1] + 1.) * 0.5;
    Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t
}

fn main() {
    const NX: u32 = 200;
    const NY: u32 = 100;
    const NS: u32 = 100;

    let R: f64 = (PI / 4.).cos();

    let mut gen = OsRng::new().unwrap();

    let sphere1 = Sphere::new(Vec3::new(0., 0., -1.), 0.5, Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let sphere4 = Sphere::new(Vec3::new(0., -100.5, -1.), 100., Lambertian::new(Vec3::new(0.8, 0.8, 0.)));
    let sphere3 = Sphere::new(Vec3::new(1., 0., -1.), 0.5, Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.));
    let sphere2 = Sphere::new(Vec3::new(-1., 0., -1.), 0.5, Dielectric::new(1.5));
    
    let world = World::new(vec![
                           Box::new(sphere1),
                           Box::new(sphere2),
                           Box::new(sphere3),
                           Box::new(sphere4)
                          ]);

    
    let camera = Camera::new(Vec3::new(-2., 2., 1.),
                             Vec3::new(0., 0., -1.),
                             Vec3::new(0., 1., 0.),
                             20., NX as f64 / NY as f64);

    let mut file = File::create("test.ppm").unwrap();

    file.write_fmt(format_args!("P3\n{} {}\n255\n", NX, NY)).unwrap();
    for j in (0..NY).rev() {
        println!("{}", j);
        for i in 0..NX {
            let mut col = Vec3::new(0., 0., 0.);
            for _ in 0..NS {
                let rand_u: f64 = gen.gen();
                let rand_v: f64 = gen.gen();
                let u = ((i as f64) + rand_u) / (NX as f64);
                let v = ((j as f64) + rand_v) / (NY as f64);

                let r = camera.get_ray(u, v);

                col += colour(r, &world, 0);
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
