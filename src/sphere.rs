use crate::hitable::*;
use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = Vec3::dot(r.direction, r.direction);
        let b = Vec3::dot(oc, r.direction);
        let c = Vec3::dot(oc, oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_paramter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_paramter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }

        return false;
    }
}
