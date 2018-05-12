use std::f64::consts::PI;

use vec3::Vec3;
use ray::Ray;

pub struct Camera {
   lower_left: Vec3,
   horizontal: Vec3,
   vertical: Vec3,
   origin: Vec3
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * PI / 180.;
        let half_height = (theta/2.).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = Vec3::cross(&vup, &w).unit_vector();
        let v = Vec3::cross(&w, &u);

        let origin = lookfrom;
        let lower_left = origin - u * half_width - v * half_height - w;
        let horizontal = u * 2. * half_width;
        let vertical = v * 2. * half_height;

        Camera {
            lower_left,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left
                              + (self.horizontal * u)
                              + (self.vertical * v)
                              - self.origin)
    }
}
