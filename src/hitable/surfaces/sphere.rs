use ray::Ray;
use vec3::Vec3;
use hitable::{HitRecord, Hitable};

#[derive(Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.squared_length();
        let b = 2. * Vec3::dot(&ray.direction, &(ray.origin - self.center));
        let rad_squared = self.radius * self.radius;
        let c = (oc).squared_length() - rad_squared;

        let discriminant = b*b - 4. * a * c;

        if discriminant.is_sign_negative() {
            return None;
        }

        let temp = (-b - discriminant.sqrt()) / (2. * a);
        if temp < t_max && temp > t_min {
            let p = ray.point_at_parameter(temp);
            let normal = (p - self.center).unit_vector();
            let hrec = HitRecord::new(temp, p, normal);
            return Some(hrec);
        }

        let temp = (-b + discriminant.sqrt()) / (2. * a);
        if temp < t_max && temp > t_min {
            let p = ray.point_at_parameter(temp);
            let normal = (p - self.center).unit_vector();
            let hrec = HitRecord::new(temp, p, normal);
            return Some(hrec);
        }

        None
    }
}
