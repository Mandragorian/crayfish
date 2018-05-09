use ray::Ray;
use vec3::Vec3;

pub mod surfaces;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, normal: Vec3) -> HitRecord {
        HitRecord {
            t,
            p,
            normal,
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

type BoxedHitable = Box<Hitable>;

pub struct World {
    pub list: Vec<BoxedHitable>
}

impl World {
    pub fn new(list: Vec<BoxedHitable>) -> World {
        World {
            list,
        }
    }
}

impl Hitable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let get_closer_hit = |(acc, closest_so_far), item: &BoxedHitable| {
            let res = item.hit(ray, t_min, closest_so_far);
            if let Some(hrec) = res {
                return (Some(hrec), hrec.t);
            }
            (acc, closest_so_far)
        };
        self.list.iter().fold((None, t_max), get_closer_hit).0
    }
}
