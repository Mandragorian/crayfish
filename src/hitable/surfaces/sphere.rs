use ray::Ray;
use vec3::Vec3;
use hitable::{HitRecord, Hitable};
use material::Material;

#[derive(Copy, Clone)]
pub struct Sphere<T>
where
    T: Material + Copy + Clone
{
    center: Vec3,
    radius: f64,
    material: T,
}

impl<T> Sphere<T>
where
    T: Material + Copy + Clone
{
    pub fn new(center: Vec3, radius: f64, material: T) -> Sphere<T> {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<'a, T> Hitable<'a> for Sphere<T>
where
    T: Material + Copy + Clone + 'a
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        let oc = ray.origin - self.center;
        let a = ray.direction.squared_length();
        let b = Vec3::dot(&ray.direction, &oc);
        let rad_squared = self.radius * self.radius;
        let c = (oc).squared_length() - rad_squared;

        let discriminant = b*b - a * c;

        if discriminant.is_sign_negative() {
            return None;
        }

        let temp = (-b - discriminant.sqrt()) / a;
        if temp < t_max && temp > t_min {
            let p = ray.point_at_parameter(temp);
            let normal = (p - self.center).unit_vector();
            let hrec = HitRecord::new(temp, p, normal, Box::new(self.material));
            return Some(hrec);
        }

        let temp = (-b + discriminant.sqrt()) / a;
        if temp < t_max && temp > t_min {
            let p = ray.point_at_parameter(temp);
            let normal = (p - self.center).unit_vector();
            let hrec = HitRecord::new(temp, p, normal, Box::new(self.material));
            return Some(hrec);
        }

        None
    }
}
