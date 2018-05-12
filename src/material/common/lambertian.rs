use vec3::Vec3;
use ray::Ray;
use material::Material;
use hitable::HitRecord;
use common::random_in_unit_sphere;

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian {
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hrec: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = hrec.p + hrec.normal + random_in_unit_sphere();
        Some((self.albedo, Ray::new(hrec.p, target - hrec.p)))
    }
}
