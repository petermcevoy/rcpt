use crate::vec3::*;
use crate::ray::*;
use crate::ray;
use crate::hitable::*;
use std::rc::Rc;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_recrod: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Vec3
}
impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + ray::random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;
        return true;
    }
}

pub const ERRORMAT: Lambertian = Lambertian{albedo: Vec3::ERROR};

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = ray::reflect(r_in.direction.make_unit_vector(), rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz*ray::random_in_unit_sphere());
        *attenuation = self.albedo;
        return Vec3::dot(scattered.direction, rec.normal) > 0.0;
    }
}

pub struct Dielectric {
    pub ref_idx: f64
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = ray::reflect(r_in.direction, rec.normal);

        *attenuation = Vec3::ONES;

        let outward_normal;
        let ni_over_nt;
        let cosine;
        if Vec3::dot(r_in.direction, rec.normal) > 0.0 {
            outward_normal = -1.0*rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * Vec3::dot(r_in.direction, rec.normal) / r_in.direction.length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -1.0*Vec3::dot(r_in.direction, rec.normal) / r_in.direction.length();
        }

        match ray::refract(r_in.direction, outward_normal, ni_over_nt) {
            Some(refracted) => {
                let reflect_prob = ray::schlick(cosine, self.ref_idx);
                if rand::random::<f64>() < reflect_prob {
                    *scattered = Ray::new(rec.p, reflected);
                } else {
                    *scattered = Ray::new(rec.p, refracted);
                }
            },
            None => {
                *scattered = Ray::new(rec.p, reflected);
            }
        }

        return true;
    }
}
