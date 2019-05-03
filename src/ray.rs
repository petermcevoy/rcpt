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

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - 2.0*Vec3::dot(v,n)*n;
}
