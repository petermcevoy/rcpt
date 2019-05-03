use crate::vec3::*;
use crate::ray::*;
use crate::hitable::*;
use std::rc::Rc;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_recrod: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Vec3
}
impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;
        return true;
    }
}

pub const ERRORMAT: Lambertian = Lambertian{albedo: Vec3::ERROR};

pub struct Metal {
    pub albedo: Vec3
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(r_in.direction.make_unit_vector(), rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        return Vec3::dot(scattered.direction, rec.normal) > 0.0;
    }
}
