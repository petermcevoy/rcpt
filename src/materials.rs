use crate::Vec3;
use crate::ray::{Ray, random_in_unit_sphere, random_cosine_direction, UVW, schlick, reflect, refract};
use crate::hitable::Hit;
use crate::{PDF, CosinePDF};
use crate::core::*;

pub trait Material: Sync {
    fn scatter(&self, r_in: &Ray, rec: &Hit) -> Option<ScatterRecord>;
    fn emitted(&self, r_in: &Ray, rec: &Hit, u: Real, v: Real, p: Vec3) -> Spectrum; //Vec3 { return Vec3::ZEROS; }
    fn scattering_pdf(&self, r_in: &Ray, rec: &Hit, scattered: &Ray) -> Real {1.0}
    fn apply_diffuse(&self, spec_in: &Spectrum) -> Spectrum { Spectrum::default() }
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
    fn apply_diffuse(&self, spec_in: &Spectrum) -> Spectrum { 
        self.albedo * spec_in
    }
}

// This can be implemented with a normal matrix. 
// Or perhaps a list of paramters for gaussian functions that describe similar.
pub struct GaussRecord {
    pub lambda_in: Real,
    pub lambda_out: Real,

    pub amplitude_out: Real,
    //pub amplitude_in: Real, ????
    
    pub sigma_in: Real,
    pub sigma_out: Real,
}
impl GaussRecord {
    fn eval(&self, lambda_i: Real, lambda_o: Real) -> Real {
        // Multivariate gaussian.
        // Need: Sigma_inv, pos
        let x = [lambda_i, lambda_o];
        let mu = [self.lambda_in, self.lambda_out];

        // cov = [
        //      sigma_in, 0,
        //      0, sigma_out,
        // ]
        // cov_inv = [
        //     1.0/sigma_in, 0,
        //     0, 1.0/sigma_out
        // ]
        
        let s_i = self.sigma_in*self.sigma_in;
        let s_o = self.sigma_out*self.sigma_out;
        
        let cov_det = s_i * s_o;
        let n = ((2.0*PI).powf(2.0) * cov_det).sqrt();
        
        // N = (x-mu).T * cov_inv * (x-mu)
        // fac = (x-mu).T * cov_inv * (x-mu)
        let fac = 1.0/s_i * (x[0] - mu[0])*(x[0] - mu[0]) + 
                  1.0/s_o * (x[1] - mu[1])*(x[1] - mu[1]);

        return ((-fac / 2.0).exp()) * self.amplitude_out;
    }
}

pub struct LambertianReRad { 
    //pub albedo: Spectrum, 
    pub emit: Spectrum,
    pub rerad_list: Vec<GaussRecord>,
}
impl Material for LambertianReRad {
    fn scatter(&self, r_in: &Ray, rec: &Hit) -> Option<ScatterRecord> {
        Some(
            ScatterRecord{
                specular_ray: None,
                attenuation: Spectrum::default(),
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
    fn apply_diffuse(&self, spec_in: &Spectrum) -> Spectrum { 
        // NOTE: Need incomming light in order to give a correct spectrum back...
        //
        // For each sample in spec_in. Evaluate the rerad and make new spectrum.
        //self.albedo * spec_in

        #[cfg(feature = "use_sampled_spectrum")]
        {
            let mut spec_out: Spectrum = Spectrum::default();
            for i_i in 0..crate::spectrum::N_SPECTRAL_SAMPLES {
                let lambda_i = crate::spectrum::SAMPLED_LAMBDA[i_i];
                let value_i = spec_in.c[i_i];

                // Looping from i_i, since we can only reradiate at wavlength 
                // at or longer than lambda_i.
                for i_o in i_i..crate::spectrum::N_SPECTRAL_SAMPLES {
                    let lambda_o = crate::spectrum::SAMPLED_LAMBDA[i_o];

                    // Evaluate GaussRecords...
                    let mut value_o = 0.0;
                    for gauss_record in self.rerad_list.iter() {
                        value_o += gauss_record.eval(lambda_i, lambda_o);
                    }

                    spec_out.c[i_o] += (value_o * value_i);
                    //println!("[lambda_i: {:.2}, lambda_o: {:.2}]:value_o: {}", lambda_i, lambda_o, value_o);
                }
            }

            return spec_out;
        }
        
        #[cfg(not(feature = "use_sampled_spectrum"))]
        Spectrum::default()
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
