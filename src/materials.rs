use crate::Vec3;
use crate::ray::{Ray, random_in_unit_sphere, random_cosine_direction, UVW, schlick, reflect, refract};
use crate::hitable::Hit;

use std::f64::consts::PI;

pub trait Material: Sync {
    fn scatter(&self, r_in: &Ray, rec: &Hit, alb: &mut Vec3, scattered: &mut Ray, pdf: &mut f64) -> bool;
    fn emitted(&self, r_in: &Ray, rec: &Hit, u: f64, v: f64, p: Vec3) -> Vec3 {
        if rec.normal.dot(r_in.direction) < 0.0 { 
            return self.emit(); 
        }
        return Vec3::ZEROS;
    }
    fn emit(&self) -> Vec3;
    fn scattering_pdf(&self, r_in: &Ray, rec: &Hit, scattered: &Ray) -> f64;
}

pub struct Lambertian { pub albedo: Vec3, pub emit: Vec3 }
impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &Hit, alb: &mut Vec3, scattered: &mut Ray, pdf: &mut f64) -> bool {
        let uvw = UVW::onb_from_w(rec.normal);
        let direction = uvw.local(random_cosine_direction());
        *scattered = Ray::new(rec.p, direction);
        *alb = self.albedo;
        *pdf = (uvw.w).dot(scattered.direction) / PI;
        return true;

        //let mut direction;
        //loop {
        //    direction = random_in_unit_sphere();
        //    if direction.dot(rec.normal) > 0.0 {break;}
        //}

        ////let target = rec.p + rec.normal + random_in_unit_sphere();
        ////*scattered = Ray{origin: rec.p, direction: (target - rec.p).make_unit_vector()};
        //*scattered = Ray{origin: rec.p, direction: direction.make_unit_vector()};
        //*alb = self.albedo;
        ////*pdf = (rec.normal.make_unit_vector()).dot(scattered.direction.make_unit_vector()) / PI;
        //*pdf = ((rec.normal.make_unit_vector()).dot(scattered.direction.make_unit_vector()) * 0.5)/PI;
        //return true;


        //*scattered = Ray::new(rec.p, direction.make_unit_vector());
        //*alb = self.albedo;
        //*pdf = (uvw.w).dot(scattered.direction) / PI;
        //return true;
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: &Hit, scattered: &Ray) -> f64 {
        let mut cosine = (rec.normal.make_unit_vector()).dot(scattered.direction.make_unit_vector());
        if cosine < 0.0 {cosine = 0.0}
        return cosine / PI;
    }
    fn emit(&self) -> Vec3 {self.emit}
}

//pub struct DiffuseLight { pub albedo: Vec3 }
//impl Material for DiffuseLight {
//    fn scatter(&self, r_in: &Ray, hit_recrod: &Hit, alb: &mut Vec3, scattered: &mut Ray, pdf: &mut f64) -> bool {
//        return false;
//    }
//    fn emitted(&self, r_in: &Ray, rec: &Hit, u: f64, v: f64, p: Vec3) -> Vec3 {
//        if rec.normal.dot(r_in.direction) < 0.0 {
//            return self.albedo;
//        }
//        return Vec3::ZEROS;
//    }
//    fn scattering_pdf(&self, r_in: &Ray, rec: &Hit, scattered: &Ray) -> f64 {
//        return 1.0 / PI;
//    }
//}
