use std::ops::{Add, Sub, Mul};
use std::result::Result;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 { e: [f64; 3] }

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3{ e: {[x, y, z]} }
    }
    pub const ZEROS: Vec3 = Vec3{ e: {[0.0, 0.0, 0.0]}};
    pub const ONES: Vec3 = Vec3{ e: {[1.0, 1.0, 1.0]}};

    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }
    pub fn r(&self) -> f64 { self.e[0] }
    pub fn g(&self) -> f64 { self.e[1] }
    pub fn b(&self) -> f64 { self.e[2] }

    fn squared_length(&self) -> f64 {
        (
            self.e[0]*self.e[0] + 
            self.e[1]*self.e[1] + 
            self.e[2]*self.e[2] 
        )
    }

    fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn make_unit_vector(&self) -> Vec3 {
        let norm = self.length();
        let v = Vec3 { e: {[
            self.e[0] / norm,
            self.e[1] / norm,
            self.e[2] / norm
        ]}};
        return v
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {e: {[
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2]
        ]}}
    }
}

impl Sub for & Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3 {e: {[
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2]
        ]}}
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {e: {[
            self.e[0]*rhs,
            self.e[1]*rhs,
            self.e[2]*rhs
        ]}}
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {e: {[
            self*rhs.e[0],
            self*rhs.e[1],
            self*rhs.e[2]
        ]}}
    }
}
