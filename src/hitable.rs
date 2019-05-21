use crate::core::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Hit {
    pub t: Real,
    pub p: Vec3,
    pub u: Real,
    pub v: Real,
    pub normal: Vec3,
    pub material: Option<Arc<Material + Send>>
}

pub trait Hitable: Sync + Send {
    fn hit(&self, r: &Ray) -> Option<Hit>;
    fn pdf_value(&self, origin: Vec3, v: Vec3) -> Real { return 0.0; }
    fn random(&self, origin: Vec3) -> Vec3 { return Vec3(1.0, 0.0, 0.0); }
}

pub type HitList = Vec<Box<Hitable>>;
impl Hitable for Vec<Box<Hitable>> {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        let mut rec: Option<Hit> = None;
        let mut closest_so_far = R_MAX;
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
