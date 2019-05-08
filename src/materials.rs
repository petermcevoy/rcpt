use crate::Vec3;
use crate::ray::{Ray, random_in_unit_sphere, schlick, reflect, refract};
use crate::hitable::Hit;

pub trait Material: Sync {
    fn scatter(&self, r_in: &Ray, hit_recrod: &Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        Vec3::ZEROS
    }
}

pub struct Lambertian { pub albedo: Vec3 }
impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;
        return true;
    }
}

pub struct DiffuseLight { pub albedo: Vec3 }
impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, rec: &Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        return false;
    }
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        return self.albedo;
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(r_in.direction.make_unit_vector(), rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz*random_in_unit_sphere());
        *attenuation = self.albedo;
        return scattered.direction.dot(rec.normal) > 0.0;
    }
}

pub struct Dielectric {
    pub ref_idx: f64
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(r_in.direction, rec.normal);

        *attenuation = Vec3::ONES;

        let outward_normal;
        let ni_over_nt;
        let cosine;
        if r_in.direction.dot(rec.normal) > 0.0 {
            outward_normal = -1.0*rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * r_in.direction.dot(rec.normal) / r_in.direction.length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -1.0*r_in.direction.dot(rec.normal) / r_in.direction.length();
        }

        match refract(r_in.direction, outward_normal, ni_over_nt) {
            Some(refracted) => {
                let reflect_prob = schlick(cosine, self.ref_idx);
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
