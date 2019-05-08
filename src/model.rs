use crate::vec::Vec3;
use crate::ray::Ray;
use crate::materials::Material;
use std::sync::Arc;


#[derive(Clone)]
pub struct Hit {
    pub t: f64,
    pub p: Vec3,
    pub u: f64,
    pub v: f64,
    pub normal: Vec3,
    pub material: Arc<Material>
}

pub trait Model: Sync + Send{
    fn hit(&self, r: &Ray) -> Option<Hit>;
}

const T_MIN: f64 = 0.0001;
const T_MAX: f64 = std::f64::MAX;

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<Material + Send>
}

impl Model for Sphere {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        let oc = r.origin - self.center;
        let a = Vec3::dot(r.direction, r.direction);
        let b = Vec3::dot(oc, r.direction);
        let c = Vec3::dot(oc, oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let mut t = (-b - discriminant.sqrt()) / a;
            if t < T_MAX && t > T_MIN {
                let p = r.point_at_paramter(t);
                return Some(Hit{
                    t,
                    p,
                    u: 0.0, //TODO
                    v: 0.0, //TODO
                    normal: (p - self.center) / self.radius,
                    material: self.material.clone(),
                });
            }
            t= (-b + discriminant.sqrt()) / a;
            if t < T_MAX && t > T_MIN {
                let p = r.point_at_paramter(t);

                return Some(Hit{
                    t,
                    p, 
                    u: 0.0, //TODO
                    v: 0.0, //TODO
                    normal: (p - self.center) / self.radius,
                    material: self.material.clone()
                });
            }
        }
        None
    }
}

pub struct XYRect { 
    x0: f64, x1: f64, y0: f64, y1: f64, k: f64,
    pub material: Arc<Material + Send>
}

impl Model for XYRect {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        let t = (self.k-r.origin.z()) / r.direction.z();

        if (t < T_MIN || t > T_MAX) { 
            return None 
        }

        let x = r.origin.x() + t*r.direction.x();
        let y = r.origin.y() + t*r.direction.y();

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 { 
            return None
        }

        Some(Hit{
            t,
            p: r.point_at_paramter(t),
            u: (x-self.x0) / (self.x1-self.x0),
            v: (y-self.y0) / (self.y1-self.y0),
            normal: Vec3(0.0, 0.0, 1.0),
            material: self.material.clone(),
        })
    }
}


impl Model for Vec<Box<Model>> {
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
}


