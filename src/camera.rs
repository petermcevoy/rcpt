use crate::vec3::*;
use crate::ray::*;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f64,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = 2.0*Vec3::new(rand::random::<f64>(), rand::random::<f64>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if Vec3::dot(p,p) < 1.0 {
            return p;
        }
    }
}

impl Camera {
    pub const DEFAULT: Camera = Camera{
        lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0),
        origin: Vec3::new(0.0, 0.0, 0.0),
        lens_radius: 1.0,
        u: Vec3::new(1.0, 0.0, 0.0),
        v: Vec3::new(0.0, 1.0, 0.0),
        w: Vec3::new(0.0, 0.0, -1.0),
    };

    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64,
               aperture: f64, focus_dist: f64) -> Camera {
        let theta = vfov*std::f64::consts::PI/180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect*half_height;

        let w = (lookfrom - lookat).make_unit_vector();
        let u = Vec3::cross(vup, w).make_unit_vector();
        let v = Vec3::cross(w, u);

        Camera {
            lower_left_corner: lookfrom - half_width*focus_dist*u - half_height*focus_dist*v - focus_dist*w,
            horizontal: 2.0*half_width*focus_dist*u,
            vertical: 2.0*half_height*focus_dist*v,
            origin: lookfrom,
            lens_radius: aperture / 2.0,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius*random_in_unit_disk();
        let offset = self.u*rd.x() + self.v*rd.y();
        return Ray{
            origin: self.origin + offset, 
            direction: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset
        };
    }
}
