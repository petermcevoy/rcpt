use std::ops::{Add, AddAssign, Sub, Mul, Div, DivAssign, Index};

#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3(x, y, z)
    }
    pub const ZEROS: Vec3 = Vec3(0.0, 0.0, 0.0);
    pub const ONES: Vec3 = Vec3(1.0, 1.0, 1.0);
    pub const RED: Vec3 = Vec3(1.0, 0.0, 0.0);
    pub const GREEN: Vec3 = Vec3(0.0, 1.0, 0.0);
    pub const BLUE: Vec3 = Vec3(0.0, 0.0, 1.0);
    pub const ERROR: Vec3 = Vec3(1.0, 0.0, 1.0);

    pub fn x(&self) -> f64 { self.0 }
    pub fn y(&self) -> f64 { self.1 }
    pub fn z(&self) -> f64 { self.2 }
    pub fn r(&self) -> f64 { self.0 }
    pub fn g(&self) -> f64 { self.1 }
    pub fn b(&self) -> f64 { self.2 }

    pub fn dot(self, b: Vec3) -> f64 {
        return 
            self.0 * b.0 +
            self.1 * b.1 +
            self.2 * b.2;
    }
    
    pub fn cross(self, b: Vec3) -> Vec3 {
        return Vec3::new(
            self.1*b.2 - self.2*b.1,
            -(self.0*b.2 - self.2*b.0),
            self.0*b.1 - self.1*b.0
            );
    }
    
    pub fn squared_length(&self) -> f64 {
        (
            self.0*self.0 + 
            self.1*self.1 + 
            self.2*self.2 
        )
    }
    
    pub fn norm(&self) -> f64 {
        return self.squared_length();
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn make_unit_vector(&self) -> Vec3 {
        let norm = self.length();
        Vec3(
            self.0 / norm,
            self.1 / norm,
            self.2 / norm
        )
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2
        )
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3(
            self.0 + other.0, 
            self.1 + other.1, 
            self.2 + other.2, 
        );
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2
        )
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self.0*rhs.0,
            self.1*rhs.1,
            self.2*rhs.2
        )
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3(
            self.0*rhs,
            self.1*rhs,
            self.2*rhs
        )
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self*rhs.0,
            self*rhs.1,
            self*rhs.2
        )
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3(
            self.0/rhs,
            self.1/rhs,
            self.2/rhs
        )
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Vec3(
            self.0/rhs,
            self.1/rhs,
            self.2/rhs
        )
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => return &self.0,
            1 => return &self.1,
            2 => return &self.2,
            _ => panic!("Out of bounds.")
        }
    }
}
