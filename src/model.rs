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
    pub material: Option<Arc<Material + Send>>
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
    pub material: Option<Arc<Material + Send>>
}
impl Plane {
    pub fn new() -> Plane {
        Plane{
            origin: Vec3::ZEROS,
            normal: Vec3(0.0, 0.0, 1.0),
            rot_around_normal: 0.0,
            width: 1.0,
            height: 1.0,
            material: None
        }
    }
}

impl Hitable for Plane {
    fn hit(&self, r: &Ray) -> Option<Hit> {
		let local_normal = Vec3(0.0, 0.0, 1.0);

        let irot = Quaternion::rot_from_vecs(self.normal, local_normal);
        let local_ray = Ray{
            origin: irot.transform_vec(r.origin - self.origin),
            direction: irot.transform_vec(r.direction),
        };
        
        // Check if we intersect the infinite plane.
        let denom = local_normal.dot(local_ray.direction);
        if denom < 0.0 {
            let t = (-1.0*local_ray.origin).dot(local_normal) / denom;

			if t > 0.0 {
                // Check if we are in bounds.
                let local_p = local_ray.point_at_paramter(t);

                if local_p.x().abs() < self.width/2.0 && local_p.y().abs() < self.height / 2.0 {
                    return Some(Hit{
                        t,
                        p: r.point_at_paramter(t), 
                        u: 0.0, //TODO
                        v: 0.0, //TODO
                        normal: self.normal,
                        material: self.material.clone()
                    });
                }
		    }
		}
		None
    }
}

pub struct Cuboid {
    pub origin: Vec3,
    pub rot: Quaternion,
    pub size: Vec3,
    pub material: Option<Arc<Material + Send>>
}

impl Hitable for Cuboid {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        // The local cuboid consits of 4 planes.
        // It has origin in (0,0,0) 
        
        let irot = self.rot.inv();
        let local_ray = Ray{
            origin: irot.transform_vec(r.origin - self.origin),
            direction: irot.transform_vec(r.direction),
        };

        fn transform_hit_to_global(local_rec: Option<Hit>, global_ray: &Ray, c: &Cuboid) -> Option<Hit> {
            if let Some(mut rec) = local_rec {
                rec.material = c.material.clone();
                rec.p = global_ray.point_at_paramter(rec.t);
                return Some(rec);
            }
            None
        }

        let mut temp_plane = Plane::new();
		let mut temp_plane_hit;
        
        // Hit front?
        temp_plane.origin = Vec3(0.0, 0.0, self.size.z()/2.0);
        temp_plane.normal = Vec3(0.0, 0.0, 1.0);
        temp_plane.width = self.size.x();
        temp_plane.height = self.size.y();
        temp_plane_hit = transform_hit_to_global(temp_plane.hit(&local_ray), r, &self);
        if temp_plane_hit.is_some() {return temp_plane_hit;}
        
        // Hit back?
        temp_plane.origin = Vec3(0.0, 0.0, -self.size.z()/2.0);
        temp_plane.normal = Vec3(0.0, 0.0, -1.0);
        temp_plane.width = self.size.x();
        temp_plane.height = self.size.y();
        temp_plane_hit = transform_hit_to_global(temp_plane.hit(&local_ray), r, &self);
        if temp_plane_hit.is_some() {return temp_plane_hit;}

        // Hit left side?
        temp_plane.origin = Vec3(-self.size.x()/2.0, 0.0, 0.0);
        temp_plane.normal = Vec3(-1.0, 0.0, 0.0);
        temp_plane.width = self.size.z();
        temp_plane.height = self.size.y();
        temp_plane_hit = transform_hit_to_global(temp_plane.hit(&local_ray), r, &self);
        if temp_plane_hit.is_some() {return temp_plane_hit;}

        // Hit right side?
        temp_plane.origin = Vec3(self.size.x()/2.0, 0.0, 0.0);
        temp_plane.normal = Vec3(1.0, 0.0, 0.0);
        temp_plane.width = self.size.z();
        temp_plane.height = self.size.y();
        temp_plane_hit = transform_hit_to_global(temp_plane.hit(&local_ray), r, &self);
        if temp_plane_hit.is_some() {return temp_plane_hit;}
        
        // Hit top?
        temp_plane.origin = Vec3(0.0, self.size.y()/2.0, 0.0);
        temp_plane.normal = Vec3(0.0, 1.0, 0.0);
        temp_plane.width = self.size.x();
        temp_plane.height = self.size.z();
        temp_plane_hit = transform_hit_to_global(temp_plane.hit(&local_ray), r, &self);
        if temp_plane_hit.is_some() {return temp_plane_hit;}
        
        // Hit bottom?
        temp_plane.origin = Vec3(0.0, -self.size.y()/2.0, 0.0);
        temp_plane.normal = Vec3(0.0, -1.0, 0.0);
        temp_plane.width = self.size.x();
        temp_plane.height = self.size.z();
        temp_plane_hit = transform_hit_to_global(temp_plane.hit(&local_ray), r, &self);
        if temp_plane_hit.is_some() {return temp_plane_hit;}

        None

    }
}
