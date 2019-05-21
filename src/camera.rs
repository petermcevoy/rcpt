use crate::core::*;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: Real,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = 2.0*Vec3::new(rand::random::<Real>(), rand::random::<Real>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.dot(p) < 1.0 {
            return p;
        }
    }
}

impl Camera {
    pub fn none() -> Camera {
        Camera::new(Vec3(0.0, 0.0, 5.0), Vec3::ZEROS, Vec3(0.0, 1.0, 0.0), 45.0, 0.5, 0.1, 5.0)
    }

    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: Real, aspect: Real,
               aperture: Real, focus_dist: Real) -> Camera {
        let theta = vfov*PI/180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect*half_height;

        let w = (lookfrom - lookat).make_unit_vector();
        let u = vup.cross(w).make_unit_vector();
        let v = w.cross(u);

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

    pub fn get_ray(self, s: Real, t: Real) -> Ray {
        let rd = self.lens_radius*random_in_unit_disk();
        let offset = self.u*rd.x() + self.v*rd.y();
        return Ray{
            origin: self.origin + offset, 
            direction: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset
        };
    }
}
