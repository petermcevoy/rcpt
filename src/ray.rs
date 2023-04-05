use crate::core::{Real, PI};
use crate::{Hitable, Quaternion, Vec3};
use rand::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn point_at_paramter(self, t: Real) -> Vec3 {
        return self.origin + t * self.direction;
    }

    pub const NONE: Ray = Ray {
        origin: Vec3::ZEROS,
        direction: Vec3::ZEROS,
    };
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p: Vec3;
    loop {
        p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::ONES;
        if p.squared_length() < 1.0 {
            break;
        }
    }
    return p;
}

pub fn random_cosine_direction() -> Vec3 {
    let r1 = rand::random::<Real>();
    let r2 = rand::random::<Real>();
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * 2.0 * (r2.sqrt());
    let y = phi.sin() * 2.0 * (r2.sqrt());
    Vec3(x, y, z)
}

pub fn random_to_sphere(radius: Real, distance_squared: Real) -> Vec3 {
    let r1 = rand::random::<Real>();
    let r2 = rand::random::<Real>();
    //let z = 1.0 + r2*((1.0 - radius*radius/(distance_squared + 1e-5)).sqrt() - 1.0);
    let z = 1.0 + r2 * ((1.0 - radius * radius / (distance_squared)).sqrt() - 1.0);
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();
    Vec3(x, y, z)
}

#[derive(Clone, Copy, Debug)]
pub struct UVW {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}
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
        UVW { u, v, w }
    }

    pub fn local(self, a: Vec3) -> Vec3 {
        return 1.0 * a.0 * self.u + a.1 * self.v + a.2 * self.w;
    }
}
#[inline]
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - 2.0 * Vec3::dot(v, n) * n;
}

#[inline]
pub fn refract(v: Vec3, n: Vec3, ni_over_nt: Real) -> Option<Vec3> {
    let uv = v.make_unit_vector();
    let dt = Vec3::dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        return Some(refracted);
    }
    return None;
}

#[inline]
pub fn schlick(cosine: Real, ref_idx: Real) -> Real {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}

pub trait PDF {
    fn value(&self, direction: Vec3) -> Real;
    fn generate(&self) -> Vec3;
}

pub struct CosinePDF {
    pub to_world_rot: Quaternion,
    pub uvw: UVW,
}
impl CosinePDF {
    pub fn new(w: Vec3) -> CosinePDF {
        CosinePDF {
            to_world_rot: Quaternion::rot_from_vecs(Vec3(0.0, 1.0, 0.0), w),
            uvw: UVW::onb_from_w(w),
        }
    }
}
impl PDF for CosinePDF {
    fn value(&self, direction: Vec3) -> Real {
        let cosine = (direction.make_unit_vector()).dot(self.uvw.w);
        if cosine > 0.0 {
            return cosine / PI;
        }
        return 1.0; //TODO: Should be zero?
    }

    fn generate(&self) -> Vec3 {
        let cr = random_cosine_direction();
        let cval = cr.make_unit_vector();
        self.uvw.local(cval)
    }
}

pub struct HitablePDF<'a> {
    pub origin: Vec3,
    pub hitable: &'a dyn Hitable,
}
impl<'a> HitablePDF<'a> {
    pub fn new(hitable: &'a dyn Hitable, origin: Vec3) -> HitablePDF {
        HitablePDF { origin, hitable }
    }
}
impl<'a> PDF for HitablePDF<'a> {
    fn value(&self, direction: Vec3) -> Real {
        return self.hitable.pdf_value(self.origin, direction);
    }

    fn generate(&self) -> Vec3 {
        return self.hitable.random(self.origin);
    }
}

pub struct MixturePDF<'a> {
    pdfs: [&'a dyn PDF; 2],
}
impl<'a> MixturePDF<'a> {
    pub fn new(pdf1: &'a dyn PDF, pdf2: &'a dyn PDF) -> MixturePDF<'a> {
        MixturePDF { pdfs: [pdf1, pdf2] }
    }
}
impl<'a> PDF for MixturePDF<'a> {
    fn value(&self, direction: Vec3) -> Real {
        return 0.5 * self.pdfs[0].value(direction) + 0.5 * self.pdfs[1].value(direction);
    }

    fn generate(&self) -> Vec3 {
        if rand::random::<bool>() {
            return self.pdfs[0].generate();
        } else {
            return self.pdfs[1].generate();
        }
    }
}
