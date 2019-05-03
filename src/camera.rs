use crate::vec3::*;
use crate::ray::*;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3
}

impl Camera {
    pub const DEFAULT: Camera = Camera{
        lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0),
        origin: Vec3::new(0.0, 0.0, 0.0)
    };

    pub fn get_ray(self, u: f64, v: f64) -> Ray {
        return Ray{
            origin: self.origin, 
            direction: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin
        };
    }
}
