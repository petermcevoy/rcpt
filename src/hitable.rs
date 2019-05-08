use crate::Vec3;
use crate::ray::Ray;
use crate::materials::Material;
use crate::aabb::AABB;
use std::sync::Arc;

pub const T_MIN: f64 = 0.0001;
pub const T_MAX: f64 = std::f64::MAX;

#[derive(Clone)]
pub struct Hit {
    pub t: f64,
    pub p: Vec3,
    pub u: f64,
    pub v: f64,
    pub normal: Vec3,
    pub material: Arc<Material>
}

pub trait Hitable: Sync + Send {
    fn hit(&self, r: &Ray) -> Option<Hit>;
    //fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}

pub type HitList = Vec<Box<Hitable>>;
impl Hitable for Vec<Box<Hitable>> {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        let mut rec: Option<Hit> = None;
        let mut closest_so_far = std::f64::MAX;
        for item in self.iter() {
            match item.hit(&r) {
                Some(temp_rec) => {
                    if temp_rec.t < closest_so_far {
                        closest_so_far = temp_rec.t;
                        rec = Some(temp_rec);
                    }
                },
                None => {},
            }
        }
        rec
    }

    //fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
    //    if self.len() < 1 {return None}
    //    let mut bbox;
    //    match self[0].bounding_box(t0, t1) {
    //        Some(temp_bbox) => { bbox = temp_bbox; },
    //        None => { return None }
    //    }

    //    for item in self {
    //        match item.bounding_box(t0, t1) {
    //            Some(temp_bbox) => {
    //                bbox = AABB::surrounding_box(bbox, temp_bbox);
    //            },
    //            None => {return None;}
    //        }
    //    }
    //    return Some(bbox);
    //}
}
