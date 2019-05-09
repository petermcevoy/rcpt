use std::ops::{Add, AddAssign, Sub, Mul, Div, DivAssign, Index};

use super::vec::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Quaternion {
    pub w: f64, pub x: f64, pub y: f64, pub z: f64
}


impl Quaternion {
    pub const UNIT: Quaternion = Quaternion{w: 1.0, x: 0.0, y: 0.0, z:0.0};

    pub fn new() -> Quaternion {
        Quaternion::UNIT
    }
    
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
    
    pub fn from_eulerangles(eul_ypr: Vec3) -> Quaternion {
        // Yaw (z), Picth (Y), Roll (X)
        let cy = (eul_ypr.z() * 0.5).cos();
        let sy = (eul_ypr.z() * 0.5).sin();
        let cp = (eul_ypr.y() * 0.5).cos();
        let sp = (eul_ypr.y() * 0.5).sin();
        let cr = (eul_ypr.x() * 0.5).cos();
        let sr = (eul_ypr.x() * 0.5).sin();

        Quaternion{
            w: cy * cp * cr + sy * sp * sr,
            x: cy * cp * sr - sy * sp * cr,
            y: sy * cp * sr + cy * sp * cr,
            z: sy * cp * cr - cy * sp * sr,
        }
    }

    pub fn to_eulerangles(self) -> Vec3 {
        // roll (x-axis)
        let sinr = 2.0 * (self.w * self.x + self.y * self.z);
        let cosr = 1.0 - 2.0 * (self.x * self.x + self.y * self.y);
        let roll = sinr.atan2(cosr);
        
        // pitch (y-axis)
        let pitch;
        let sinp = 2.0 * (self.w * self.y + self.z * self.x);
        if sinp.abs() >= 1.0 {
            let val = std::f64::consts::PI / 2.0;
            if sinp >= 0.0 {
                pitch = val;
            } else {
                pitch = -1.0*val;
            }
        } else {
            pitch = sinp.asin();
        }

        // yaw (z-axis)
        let siny = 2.0 * (self.w * self.z + self.x * self.y);
        let cosy = 1.0 - 2.0 * (self.y * self.y + self.z * self.z);
        let yaw  = siny.atan2(cosy);

        return Vec3(roll, pitch, yaw);
    }

    // Find the rotation transform between two vectors. From v1 to v2.
    // https://stackoverflow.com/questions/1171849/finding-quaternion-representing-the-rotation-from-one-vector-to-another
    pub fn rot_from_vecs(v1: Vec3, v2: Vec3) -> Quaternion {
        let d = v1.make_unit_vector().dot(v2.make_unit_vector());
        if d > 0.9999 { // Vectors are parallel.
            return Quaternion::UNIT;
        } else if d < -0.9999 {
            return Quaternion{ w: 0.0, x: 1.0, y: 0.0, z: 0.0 };
        }
        
        let a = v1.cross(v2);
        let q = Quaternion{
            w: v1.norm() * v2.norm() + v1.dot(v2),
            x: a.0, y: a.1, z: a.2,
        };

        return q.normalize();
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
