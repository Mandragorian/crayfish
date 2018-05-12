use vec3::Vec3;
use ray::Ray;
use hitable::HitRecord;
use material::Material;
use common::{random_in_unit_sphere, reflect};

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hrec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&r_in.direction.unit_vector(), &hrec.normal);
        let scattered = Ray::new(hrec.p, reflected + random_in_unit_sphere()*self.fuzz);
        if Vec3::dot(&scattered.direction, &hrec.normal).is_sign_positive() {
            return Some((self.albedo, scattered))
        }
        None
    }
}
