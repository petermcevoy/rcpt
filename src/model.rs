use crate::vec::Vec3;
use crate::ray::Ray;
use crate::materials::Material;
use std::sync::Arc;


#[derive(Clone)]
pub struct Hit {
    pub t: f64,
    pub p: Vec3,
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
                    normal: (p - self.center) / self.radius,
                    material: self.material.clone()
                });
            }
        }
        None
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
