use ray::Ray;
use vec3::Vec3;
use hitable::HitRecord;

pub mod common;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hrec: &HitRecord) -> Option<(Vec3, Ray)>;
}
