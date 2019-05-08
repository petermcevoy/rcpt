use crate::{
    Vec3,
    Quaternion,
    ray::Ray,
    hitable::{Hit, Hitable, T_MIN, T_MAX},
    materials::Material,
    aabb::AABB,
};

use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<Material + Send>
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let mut t = (-b - discriminant.sqrt()) / a;
            if t < T_MAX && t > T_MIN {
                let p = r.point_at_paramter(t);
                return Some(Hit{
                    t,
                    p,
                    u: 0.0, //TODO
                    v: 0.0, //TODO
                    normal: (p - self.center) / self.radius,
                    material: self.material.clone(),
                });
            }
            t= (-b + discriminant.sqrt()) / a;
            if t < T_MAX && t > T_MIN {
                let p = r.point_at_paramter(t);

                return Some(Hit{
                    t,
                    p, 
                    u: 0.0, //TODO
                    v: 0.0, //TODO
                    normal: (p - self.center) / self.radius,
                    material: self.material.clone()
                });
            }
        }
        None
    }

    //fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
    //    let bbox = AABB::new(
    //            self.center - self.radius*Vec3::ONES, 
    //            self.center + self.radius*Vec3::ONES
    //        );
    //    Some(bbox)
    //}
}

pub struct Plane {
    pub origin: Vec3,
    pub normal: Vec3, 
    pub rot_around_normal: f64, // Axis-Angle rotation around normal
    pub width: f64,
    pub height: f64,
    pub material: Arc<Material + Send>
}

impl Hitable for Plane {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        let rot = Quaternion::from_axisangle(self.normal*self.rot_around_normal);
        let irot = rot.inv();
        let local_ray = Ray{
            origin: irot.transform_vec(r.origin - self.origin),
            direction: irot.transform_vec(r.direction),
        };

        let normal = Vec3(1.0, 0.0, 0.0);
        let denom = normal.dot(local_ray.direction);

        // Check if we intersect the infinite plane.
        if denom >= T_MIN {
            let t = local_ray.origin.dot(normal) / denom;
            return Some(Hit{
                t,
                p: r.point_at_paramter(t), 
                u: 0.0, //TODO
                v: 0.0, //TODO
                normal: self.normal,
                material: self.material.clone()
            });
        } else {
            None
        }
    }
}

pub struct XYRect { 
    pub x0: f64, pub x1: f64, pub y0: f64, pub y1: f64, pub k: f64,
    pub material: Arc<Material + Send>
}

impl Hitable for XYRect {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        let t = (self.k-r.origin.z()) / r.direction.z();

        if t < T_MIN || t > T_MAX { 
            return None 
        }

        let x = r.origin.x() + t*r.direction.x();
        let y = r.origin.y() + t*r.direction.y();

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 { 
            return None
        }

        Some(Hit{
            t,
            p: r.point_at_paramter(t),
            u: (x-self.x0) / (self.x1-self.x0),
            v: (y-self.y0) / (self.y1-self.y0),
            normal: Vec3(0.0, 0.0, 1.0),
            material: self.material.clone(),
        })
    }
}

pub struct FlippedNormal { pub hitable_ref: Box<Hitable> }
impl Hitable for FlippedNormal {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        match self.hitable_ref.hit(r) {
            Some(rec) => { 
                let mut temp_rec = rec.clone();
                temp_rec.normal = -1.0*rec.normal;
                return Some(temp_rec) 
            },
            None => return None
        }
    }
}

pub struct XZRect { 
    pub x0: f64, pub x1: f64, pub z0: f64, pub z1: f64, pub k: f64,
    pub material: Arc<Material + Send>
}

impl Hitable for XZRect {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        let t = (self.k-r.origin.y()) / r.direction.y();

        if t < T_MIN || t > T_MAX { 
            return None 
        }

        let x = r.origin.x() + t*r.direction.x();
        let z = r.origin.z() + t*r.direction.z();

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 { 
            return None
        }

        Some(Hit{
            t,
            p: r.point_at_paramter(t),
            u: (x-self.x0) / (self.x1-self.x0),
            v: (z-self.z0) / (self.z1-self.z0),
            normal: Vec3(0.0, -1.0, 0.0),
            material: self.material.clone(),
        })
    }
}
pub struct YZRect { 
    pub y0: f64, pub y1: f64, pub z0: f64, pub z1: f64, pub k: f64,
    pub material: Arc<Material + Send>
}

impl Hitable for YZRect {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        let t = (self.k-r.origin.x()) / r.direction.x();

        if t < T_MIN || t > T_MAX { 
            return None 
        }

        let y = r.origin.y() + t*r.direction.y();
        let z = r.origin.z() + t*r.direction.z();

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 { 
            return None
        }

        Some(Hit{
            t,
            p: r.point_at_paramter(t),
            u: (y-self.y0) / (self.y1-self.y0),
            v: (z-self.z0) / (self.z1-self.z0),
            normal: Vec3(1.0, 0.0, 0.0),
            material: self.material.clone(),
        })
    }
}
