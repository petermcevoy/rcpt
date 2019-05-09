use std::ops::{Add, AddAssign, Sub, Mul, Div, DivAssign, Index};

use super::vec::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Quaternion {
    pub w: f64, pub x: f64, pub y: f64, pub z: f64
}

impl Quaternion {
    fn conjugate(self) -> Quaternion {
        Quaternion{
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    fn norm(self) -> f64 {
        return (self.w*self.w + self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
    }

    fn normalize(self) -> Quaternion {
        let n = self.norm();
        if n < 0.0001 {
            return Quaternion{w: 1.0, x: 0.0, y: 0.0, z: 0.0};
        }
        Quaternion{
            w: self.w / n,
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }

    pub fn inv(self) -> Quaternion {
        return self.normalize().conjugate();
    }
    
    pub fn transform_vec(self, v: Vec3) -> Vec3 {
        let qn = self.normalize();
        let u = Vec3(qn.x, qn.y, qn.z);
        let s = qn.w;

        let vprime = 
            (2.0*u.dot(v)) * u +
            (s*s - u.dot(u)) * v + 
            (2.0*s) * u.cross(v);
        return vprime;
    }

    pub fn from_axisangle(aa: Vec3) -> Quaternion {
        let axis = aa.make_unit_vector();
        let angle = aa.norm();
        let s = (angle/2.0).sin();

        Quaternion {
            w: (angle/2.0).cos(),
            x: axis.0 * s,
            y: axis.1 * s,
            z: axis.2 * s
        }
    }

    // Find the rotation transform between two vectors. From v1 to v2.
    // https://math.stackexchange.com/questions/2356649/how-to-find-the-quaternion-representing-the-rotation-between-two-3-d-vectors
    pub fn rot_from_vecs(v1: Vec3, v2: Vec3) -> Quaternion {
        let n = v1.cross(v2).make_unit_vector();
        let angle = (v1.cross(v2).norm() / v1.dot(v2)).atan();

        Quaternion::from_axisangle(n*angle)
    }
}

impl Add for Quaternion {
    type Output = Quaternion;
    fn add(self, other: Quaternion) -> Quaternion {
        Quaternion{
            w: self.w + other.w,
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Quaternion {
    type Output = Quaternion;
    fn sub(self, other: Quaternion) -> Quaternion {
        Quaternion{
            w: self.w - other.w,
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;
    fn mul(self, other: Quaternion) -> Quaternion {
        Quaternion{
            w: self.w*other.w - self.x*other.x - self.y*other.y - self.z*other.z,
            x: self.w*other.w + self.x*other.x - self.y*other.y - self.z*other.z,
            y: self.w*other.w - self.x*other.x + self.y*other.y + self.z*other.z,
            z: self.w*other.w + self.x*other.x - self.y*other.y + self.z*other.z,
        }
    }
}

impl Mul<f64> for Quaternion {
    type Output = Quaternion;
    fn mul(self, other: f64) -> Quaternion {
        Quaternion{
            w: self.w * other,
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
