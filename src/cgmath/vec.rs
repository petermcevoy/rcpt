use std::ops::{Add, AddAssign, Sub, Mul, Div, DivAssign, Index};
pub use crate::core::Real;

#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub Real, pub Real, pub Real);

#[derive(Debug, Copy, Clone)]
pub enum Vec3Axis { X, Y, Z }
impl Vec3Axis {
    pub fn iter() -> std::slice::Iter<'static, Vec3Axis> {
        static AXES: [Vec3Axis; 3] = [Vec3Axis::X, Vec3Axis::Y, Vec3Axis::Z];
        AXES.into_iter()
    }
    pub fn to_index(&self) -> usize {
        match self {
            Vec3Axis::X => 0,
            Vec3Axis::Y => 1,
            Vec3Axis::Z => 2,
        }
    }
}

impl Vec3 {
    pub const fn new(x: Real, y: Real, z: Real) -> Vec3 {
        Vec3(x, y, z)
    }
    pub const ZEROS: Vec3 = Vec3(0.0, 0.0, 0.0);
    pub const ONES: Vec3 = Vec3(1.0, 1.0, 1.0);
    pub const RED: Vec3 = Vec3(1.0, 0.0, 0.0);
    pub const GREEN: Vec3 = Vec3(0.0, 1.0, 0.0);
    pub const BLUE: Vec3 = Vec3(0.0, 0.0, 1.0);
    pub const ERROR: Vec3 = Vec3(1.0, 0.0, 1.0);

    pub fn x(&self) -> Real { self.0 }
    pub fn y(&self) -> Real { self.1 }
    pub fn z(&self) -> Real { self.2 }
    pub fn r(&self) -> Real { self.0 }
    pub fn g(&self) -> Real { self.1 }
    pub fn b(&self) -> Real { self.2 }

    pub fn get_axis(&self, a: Vec3Axis) -> Real {
        match a {
            Vec3Axis::X => self.0,
            Vec3Axis::Y => self.1,
            Vec3Axis::Z => self.2,
        }
    }
    pub fn set_axis(&mut self, a: Vec3Axis, val: Real) -> Self {
        match a {
            Vec3Axis::X => self.0 = val,
            Vec3Axis::Y => self.1 = val,
            Vec3Axis::Z => self.2 = val,
        };
        *self
    }

    pub fn dot(self, b: Vec3) -> Real {
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
    
    pub fn squared_length(&self) -> Real {
        (
            self.0*self.0 + 
            self.1*self.1 + 
            self.2*self.2 
        )
    }
    
    pub fn norm(&self) -> Real {
        return self.length();
    }

    pub fn length(&self) -> Real {
        self.squared_length().sqrt()
    }

    pub fn make_unit_vector(&self) -> Vec3 {
        let norm = self.length();
        if norm < 0.0001 {
            return Vec3(0.0, 0.0, 0.0);
        }

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
impl Mul<Real> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Real) -> Vec3 {
        Vec3(
            self.0*rhs,
            self.1*rhs,
            self.2*rhs
        )
    }
}
impl Mul<Vec3> for Real {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self*rhs.0,
            self*rhs.1,
            self*rhs.2
        )
    }
}

impl Div<Real> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Real) -> Vec3 {
        Vec3(
            self.0/rhs,
            self.1/rhs,
            self.2/rhs
        )
    }
}
impl DivAssign<Real> for Vec3 {
    fn div_assign(&mut self, rhs: Real) {
        *self = Vec3(
            self.0/rhs,
            self.1/rhs,
            self.2/rhs
        )
    }
}

impl Index<usize> for Vec3 {
    type Output = Real;

    fn index(&self, i: usize) -> &Real {
        match i {
            0 => return &self.0,
            1 => return &self.1,
            2 => return &self.2,
            _ => panic!("Out of bounds.")
        }
    }
}
