use crate::hitable::*;
use crate::material::*;
use crate::vec3::Vec3;
use crate::ray::Ray;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<Material>
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = Vec3::dot(r.direction, r.direction);
        let b = Vec3::dot(oc, r.direction);
        let c = Vec3::dot(oc, oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let mut t = (-b - discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.point_at_paramter(t);
                return Some(HitRecord{
                    t,
                    p,
                    normal: (p - self.center) / self.radius,
                    material: self.material.clone(),
                });
            }
            t= (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.point_at_paramter(t);

                return Some(HitRecord{
                    t,
                    p, 
                    normal: (p - self.center) / self.radius,
                    material: self.material.clone()
                });
            }
        }

        return None;
    }
}
