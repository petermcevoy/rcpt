use crate::{
    Vec3, Hitable, Quaternion
};
use rand::prelude::*;

use std::f64::consts::PI;

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
    let mut rng = rand::thread_rng();
    let mut p : Vec3;
    loop {
        p = 2.0*Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::ONES;
        if p.squared_length() < 1.0 {break;}
    }
    return p;
}

pub fn random_cosine_direction() -> Vec3 {
    let r1 = rand::random::<f64>();
    let r2 = rand::random::<f64>();
    let z = (1.0-r2).sqrt();
    let phi = 2.0*PI*r1;
    let x = phi.cos() * 2.0 * (r2.sqrt());
    let y = phi.sin() * 2.0 * (r2.sqrt());
    Vec3(x, y, z)
}

#[derive(Clone, Copy, Debug)]
pub struct UVW { pub u: Vec3, pub v: Vec3, pub w: Vec3 }
impl UVW {
    pub fn onb_from_w(n: Vec3) -> UVW {
        let w = n.make_unit_vector();

        let a;
        if (w.0).abs() > 0.9 {
            a = Vec3(0.0, 1.0, 0.0);
        } else {
            a = Vec3(1.0, 0.0, 0.0);
        }

        let v = (w.cross(a)).make_unit_vector();
        let u = w.cross(v);
        UVW{u, v, w}
    }

    pub fn local(self, a: Vec3) -> Vec3 {
        return 1.0*a.0*self.u + a.1*self.v + a.2*self.w;
    }
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

pub trait PDF {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosinePDF { pub to_world_rot: Quaternion, pub uvw: UVW }
impl CosinePDF{
    pub fn new(w: Vec3) -> CosinePDF { CosinePDF{ to_world_rot: Quaternion::rot_from_vecs(Vec3(0.0, 1.0, 0.0), w), uvw: UVW::onb_from_w(w) } }
}
impl PDF for CosinePDF {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine = (direction.make_unit_vector()).dot(self.uvw.w);
        if cosine > 0.0 {
            return cosine / PI;
        } 
        return 1.0; //TODO: Should be zero?
    }

    fn generate(&self) -> Vec3 {
        let cr = random_cosine_direction();
        let cval = cr.make_unit_vector();
        let val = self.uvw.local(cval);
        //println!("cr: {:?}\t cval {:?}\t val: {:?}", cr.length(), cval.length(), val.length());
        return val;
        //return self.to_world_rot.transform_vec(random_cosine_direction());
    }
}

pub struct HitablePDF<'a> { pub origin: Vec3, pub hitable: &'a dyn Hitable }
impl <'a> HitablePDF<'a> {
    pub fn new( hitable: &'a dyn Hitable, origin: Vec3) -> HitablePDF { HitablePDF{ origin, hitable } }
}
impl <'a> PDF for HitablePDF<'a> {
    fn value(&self, direction: Vec3) -> f64 {
        return self.hitable.pdf_value(self.origin, direction);
    }

    fn generate(&self) -> Vec3 {
        return self.hitable.random(self.origin);
    }
}

pub struct MixturePDF { pdfs: [Box<PDF>; 2] }
impl MixturePDF {
    pub fn new(pdf1: Box<PDF>, pdf2: Box<PDF>) -> MixturePDF {
        MixturePDF {
            pdfs: [pdf1, pdf2]
        }
    }
}
impl PDF for MixturePDF {
    fn value(&self, direction: Vec3) -> f64 {
        return 0.5*self.pdfs[0].value(direction) + 0.5*self.pdfs[1].value(direction);
    }

    fn generate(&self) -> Vec3 {
        if rand::random::<f64>() < 0.5 {
            return self.pdfs[0].generate();
        } else {
            return self.pdfs[1].generate();
        }
    }
}
