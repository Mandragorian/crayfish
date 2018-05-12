use vec3::Vec3;
use ray::Ray;
use common::{refract, reflect, schlick, rand};
use material::Material;
use hitable::HitRecord;

#[derive(Copy, Clone)]
pub struct Dielectric {
    ref_idx: f64
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric {
            ref_idx,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hrec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&r_in.direction, &hrec.normal);
        let attenuation = Vec3::new(1., 1., 1.);

        //let mut outward_normal = Vec3::new(0., 0., 0.);
        //let n_ratio: f64 = 0.;

        let dot = Vec3::dot(&r_in.direction, &hrec.normal);
        let (outward_normal, n_ratio, cosine) = if dot.is_sign_positive() {
            let outward_normal = -hrec.normal;
            let n_ratio = self.ref_idx;
            let cosine = self.ref_idx * Vec3::dot(&r_in.direction, &hrec.normal)
                         * r_in.direction.length().recip();
            (outward_normal, n_ratio, cosine)
        }
        else {
            let outward_normal = hrec.normal;
            let n_ratio = self.ref_idx.recip();
            let cosine = -Vec3::dot(&r_in.direction, &hrec.normal)
                         * r_in.direction.length().recip();
            (outward_normal, n_ratio, cosine)
        };

        let res = refract(&r_in.direction, &outward_normal, n_ratio);
        if let Some(refracted) = res {
            let reflect_probability = schlick(cosine, self.ref_idx);
            if rand() < reflect_probability {
                return Some((attenuation, Ray::new(hrec.p, reflected)));
            } else {
                return Some((attenuation, Ray::new(hrec.p, refracted)));
            }
        } else {
            return Some((attenuation, Ray::new(hrec.p, reflected)));
        }
    }
}
