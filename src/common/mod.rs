extern crate rand;

use self::rand::{OsRng,Rng};

use vec3::Vec3;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut gen = OsRng::new().unwrap();
    loop {
        let p = Vec3::new(gen.gen(), gen.gen(), gen.gen()) * 2. - Vec3::new(1., 1., 1.);
        if p.squared_length() <= 1. {
            return p
        }
    }
}

pub fn refract(v: &Vec3, n: &Vec3, n_ratio: f64) -> Option<Vec3> {
    let v_unit = v.unit_vector();
    let dt = Vec3::dot(&v_unit, n);
    let discriminant = 1. - n_ratio * n_ratio * (1. - dt * dt);
    if discriminant.is_sign_positive() {
        let refracted = (v_unit - n * dt) * n_ratio - n * discriminant.sqrt();
        return Some(refracted);
    }
    None
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * Vec3::dot(v, n) * 2.
}

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1. - ref_idx) * (1. + ref_idx).recip();
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

pub fn rand() -> f64 {
    let mut gen = OsRng::new().unwrap();
    gen.gen()
}
