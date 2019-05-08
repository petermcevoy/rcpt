use crate::ray::*;
use crate::vec3::Vec3;
use crate::material::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub u: f64,
    pub v: f64,
    pub normal: Vec3,
    pub material: Arc<Material>
}

pub trait Hitable: Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub type HitList = Vec<Box<Hitable>>;

impl Hitable for HitList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest_so_far : f64 = t_max;
        for item in self.iter() {
            match item.hit(&r, t_min, closest_so_far) {
                Some(temp_rec) => {
                    closest_so_far = temp_rec.t;
                    rec = Some(temp_rec);
                },
                None => {},
            }
        }
        return rec;
    }
}
