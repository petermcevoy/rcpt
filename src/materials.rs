use crate::Vec3;
use crate::ray::{Ray, random_in_unit_sphere, random_cosine_direction, UVW, schlick, reflect, refract};
use crate::hitable::Hit;
use crate::{PDF, CosinePDF};
use crate::core::*;

pub trait Material: Sync {
    fn scatter(&self, r_in: &Ray, rec: &Hit) -> Option<ScatterRecord>;
    //fn emitted(&self, r_in: &Ray, rec: &Hit, u: f64, v: f64, p: Vec3) -> Vec3 { return Vec3::ZEROS; }
    fn emitted(&self, r_in: &Ray, rec: &Hit, u: Real, v: Real, p: Vec3) -> Spectrum; //Vec3 { return Vec3::ZEROS; }
    fn scattering_pdf(&self, r_in: &Ray, rec: &Hit, scattered: &Ray) -> Real {1.0}
}

pub struct ScatterRecord {
    pub specular_ray: Option<Ray>,
    pub attenuation: Spectrum,
    pub pdf: Option<Box<PDF>>
}

pub struct Lambertian { pub albedo: Spectrum, pub emit: Spectrum }
impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &Hit) -> Option<ScatterRecord> {
        Some(
            ScatterRecord{
                specular_ray: None,
                attenuation: self.albedo,
                pdf: Some(Box::new(CosinePDF::new(rec.normal)))
            }
        )
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: &Hit, scattered: &Ray) -> Real {
        let cosine = (rec.normal.make_unit_vector()).dot(scattered.direction.make_unit_vector()).max(0.0);
		cosine / PI
    }
    fn emitted(&self, r_in: &Ray, rec: &Hit, u: Real, v: Real, p: Vec3) -> Spectrum {
        if rec.normal.dot(r_in.direction) < 0.0 { 
            return self.emit.clone(); 
        }
        return Spectrum::default();
    }
}

/*
pub struct Metal { pub albedo: Vec3, pub fuzz: f64 }
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &Hit) -> Option<ScatterRecord> {
        let reflected = reflect(r_in.direction.make_unit_vector(), rec.normal);
        Some(
            ScatterRecord{
                specular_ray: Some(Ray{origin: rec.p, direction: reflected + self.fuzz*random_in_unit_sphere()}),
                attenuation: self.albedo,
                pdf: None
            }
        )
    }
}*/
