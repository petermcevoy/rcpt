// BVH with AABB

use std::cmp::{min, max};
use crate::Vec3;
use crate::ray::Ray;
use crate::hitable::{Hitable, Hit};
use rand::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> AABB{ AABB{min: a, max: b} }
    pub fn min(&self) -> Vec3 {self.min}
    pub fn max(&self) -> Vec3 {self.max}
    pub fn hit(&self, r: &Ray) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let mut t0 = (self.min[a] - r.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - r.origin[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let tmin = t0.max(0.0001);
            let tmax = t1.min(std::f64::MAX);
            if tmax <= tmin { return false; }
        }
        return true;
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let small = Vec3(
                box0.min().0.min(box1.min().0),
                box0.min().1.min(box1.min().1),
                box0.min().2.min(box1.min().2),
            );
        let big = Vec3(
                box0.max().0.max(box1.max().0),
                box0.max().1.max(box1.max().1),
                box0.max().2.max(box1.max().2),
            );

        return AABB::new(small, big);
    }
}

//pub struct BVHNode {
//    left: Box<Hitable>,
//    right: Box<Hitable>,
//    bbox: AABB
//}
//impl BVHNode {
//    fn compare_box_axis(a: &Hitable, b: &Hitable) -> i8 {
//        let box_left = a.bounding_box(0, 0);
//        let box_right = b.bounding_box(0, 0);
//
//
//    }
//    pub fn bvh_node(&self, n: usize) {
//        let axis = rand::random::<f64>() * 3.0 as u8;
//        if axis == 0 {
//            // TODO sorting function...
//        }
//    }
//}
//impl Hitable for BVHNode {
//    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
//        return Some(self.bbox);
//    }
//    
//    fn hit(&self, r: &Ray) -> Option<Hit> {
//        if self.bbox.hit(r) {
//            let left_rec = self.left.hit(r);
//            let right_rec = self.left.hit(r);
//            if left_rec.is_some() && right_rec.is_some() {
//                if left_rec.as_ref().unwrap().t < right_rec.as_ref().unwrap().t {
//                    return left_rec;
//                } else {
//                    return right_rec;
//                }
//            } else if left_rec.is_some() {
//                return left_rec;
//            } else if right_rec.is_some() {
//                return right_rec;
//            } 
//        }
//        return None
//    }
//}
