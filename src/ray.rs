use crate::Vec3;

#[derive(Copy, Clone)]
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
}
