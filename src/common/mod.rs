extern crate rand;

use self::rand::{OsRng,Rng};

use vec3::Vec3;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut gen = OsRng::new().unwrap();
    loop {
        let p = Vec3::new(gen.gen(), gen.gen(), gen.gen()) * 2. - Vec3::new(1., 1., 1.);
        if p.squared_length() >= 1. {
            return p
        }
    }
}

