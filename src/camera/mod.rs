use vec3::Vec3;
use ray::Ray;

pub struct Camera {
   lower_left: Vec3,
   horizontal: Vec3,
   vertical: Vec3,
   origin: Vec3
}

impl Camera {
    pub fn new() -> Camera {
        let lower_left = Vec3::new(-2., -1., -1.);
        let horizontal = Vec3::new(4., 0., 0.);
        let vertical = Vec3::new(0., 2., 0.);
        let origin = Vec3::new(0., 0., 0.);

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
