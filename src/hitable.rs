use crate::ray::*;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3
}
impl HitRecord {
    pub const NONE: HitRecord = HitRecord{t: std::f64::MAX, p: Vec3::ZEROS, normal: Vec3::ZEROS};
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub type HitList<'a> = Vec<&'a Hitable>;

impl<'a> Hitable for HitList<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::NONE;
        let mut hit_anything = false;
        let mut closest_so_far : f64 = t_max;
        for item in self.iter() {
            if item.hit(&r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        return hit_anything;
    }
}
