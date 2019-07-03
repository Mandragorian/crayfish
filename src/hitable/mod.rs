use ray::Ray;
use vec3::Vec3;
use material::Material;

pub mod surfaces;

pub struct HitRecord<'a>
{
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Box<Material + 'a>,
}

impl<'a> HitRecord<'a>
{
    pub fn new(t: f64, p: Vec3, normal: Vec3, material: Box<Material + 'a>) -> HitRecord {
        HitRecord {
            t,
            p,
            normal,
            material,
        }
    }
}

pub trait Hitable<'a>
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>>;
}

type BoxedHitable<'a> = Box<Hitable<'a> + Sync>;

pub struct World<'a> {
    pub list: Vec<BoxedHitable<'a>>
}

impl<'a> World<'a> {
    pub fn new(list: Vec<BoxedHitable<'a>>) -> World {
        World {
            list,
        }
    }
}

impl<'a> Hitable<'a> for World<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        let get_closer_hit = |(acc, closest_so_far), item: &BoxedHitable<'a>| {
            let res = item.hit(ray, t_min, closest_so_far);
            if let Some(hrec) = res {
                let new_t = hrec.t.clone();
                return (Some(hrec), new_t);
            }
            (acc, closest_so_far)
        };
        self.list.iter().fold((None, t_max), get_closer_hit).0
    }
}
