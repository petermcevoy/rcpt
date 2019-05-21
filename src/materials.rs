use crate::core::*;
use crate::ray::{
    random_in_unit_sphere, random_cosine_direction, schlick, reflect, refract,
    PDF, CosinePDF
};

pub trait Material: Sync {
    fn scatter(&self, r_in: &Ray, rec: &Hit) -> Option<ScatterRecord>;
    fn emitted(&self, r_in: &Ray, rec: &Hit, u: Real, v: Real, p: Vec3) -> Vec3 { return Vec3::ZEROS; }
    fn scattering_pdf(&self, r_in: &Ray, rec: &Hit, scattered: &Ray) -> Real {1.0}
}

pub struct ScatterRecord {
    pub specular_ray: Option<Ray>,
    pub attenuation: Vec3,
    pub pdf: Option<Box<PDF>>
}

pub struct Lambertian { pub albedo: Vec3, pub emit: Vec3 }
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
    fn emitted(&self, r_in: &Ray, rec: &Hit, u: Real, v: Real, p: Vec3) -> Vec3 {
        if rec.normal.dot(r_in.direction) < 0.0 { 
            return self.emit; 
        }
        return Vec3::ZEROS;
    }
}

pub struct Metal { pub albedo: Vec3, pub fuzz: Real }
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &Hit) -> Option<ScatterRecord> {
        let reflected = reflect(r_in.direction.make_unit_vector(), rec.normal);
        Some(
            ScatterRecord{
                specular_ray: Some(Ray::new(rec.p, reflected + self.fuzz*random_in_unit_sphere())),
                attenuation: self.albedo,
                pdf: None
            }
        )
    }
}
