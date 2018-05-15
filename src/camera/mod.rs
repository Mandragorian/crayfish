use std::f64::consts::PI;

use vec3::Vec3;
use ray::Ray;
use common::random_in_unit_disc;

pub struct Camera {
   lower_left: Vec3,
   horizontal: Vec3,
   vertical: Vec3,
   origin: Vec3,
   u: Vec3,
   v: Vec3,
   w: Vec3,
   lens_radius: f64,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Camera {
        let lens_radius = aperture / 2.;
        let theta = vfov * PI / 180.;
        let half_height = (theta/2.).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = Vec3::cross(&vup, &w).unit_vector();
        let v = Vec3::cross(&w, &u);

        let origin = lookfrom;
        let lower_left = origin - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist;
        let horizontal = u * 2. * half_width * focus_dist;
        let vertical = v * 2. * half_height * focus_dist;

        Camera {
            lower_left,
            horizontal,
            vertical,
            origin,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disc() * self.lens_radius;
        let offset = self.u * rd.e[0] + self.v * rd.e[1];
        Ray::new(self.origin + offset, self.lower_left
                              + (self.horizontal * s)
                              + (self.vertical * t)
                              - self.origin - offset)
    }
}
