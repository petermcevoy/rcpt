use crate::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {origin: origin, direction: direction}
    }

    pub fn point_at_paramter(self, t: f64) -> Vec3 {
        return self.origin + t*self.direction
    }

    pub const NONE: Ray = Ray{origin: Vec3::ZEROS, direction: Vec3::ZEROS};
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p : Vec3;
    loop {
        p = 2.0*Vec3::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>()) - Vec3::ONES;
        if p.squared_length() < 1.0 {break;}
    }
    return p;
}

#[inline]
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - 2.0*Vec3::dot(v,n)*n;
}

#[inline]
pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.make_unit_vector();
    let dt = Vec3::dot(uv, n);
    let discriminant = 1.0 - ni_over_nt*ni_over_nt*(1.0 - dt*dt);
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n*dt) - n*discriminant.sqrt();
        return Some(refracted);
    }
    return None;
}

#[inline]
pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0*r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}

