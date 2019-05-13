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

    fn pdf_value(&self, ray_origin: Vec3, v: Vec3) -> f64 {
        match self.hit(&Ray::new(ray_origin, v)) {
            Some(rec) => {
                let area = (self.width*self.height);
                //let distance_squared = rec.t * rec.t * v.squared_length();
                //let distance_squared = v.squared_length();
                let distance_squared = rec.t * rec.t * v.squared_length();
                let cosine = ((v.make_unit_vector()).dot(rec.normal)).abs();
                return distance_squared / (cosine * area);
            },
            None => {return 0.0;},
        }
    }
    fn random(&self, ray_origin: Vec3) -> Vec3 {
        let local_random_point = Vec3(
            (rand::random::<f64>() - 0.5) * self.width,
            (rand::random::<f64>() - 0.5) * self.height,
            0.0
        );
		
        let local_normal = Vec3(0.0, 0.0, 1.0);
        let rot = Quaternion::rot_from_vecs(local_normal, self.normal);

        let global_random_point = rot.transform_vec(local_random_point) + self.origin;

        return global_random_point - ray_origin;
    }
}

pub struct Cuboid {
    pub origin: Vec3,
    pub rot: Quaternion,
    pub size: Vec3,
    pub material: Option<Arc<Material + Send>>
}

impl Cuboid {
    fn local_plane_hit(&self, local_ray: &Ray, return_plane: Option<&mut Plane>) -> Option<Hit> {
        let mut temp_plane = Plane::new();
		let mut temp_plane_hit;

        fn assign_plane_if_some(plane: Plane, plane_ref_option: Option<&mut Plane>) {
            if let Some(plane_ref) = plane_ref_option {
                *plane_ref = plane;
            }
        }

        // Hit front?
        temp_plane.origin = Vec3(0.0, 0.0, self.size.z()/2.0);
        temp_plane.normal = Vec3(0.0, 0.0, 1.0);
        temp_plane.width = self.size.x();
        temp_plane.height = self.size.y();
        temp_plane_hit = temp_plane.hit(&local_ray);
        if temp_plane_hit.is_some() {
            assign_plane_if_some(temp_plane, return_plane);
            return temp_plane_hit;
        }
        
        // Hit back?
        temp_plane.origin = Vec3(0.0, 0.0, -self.size.z()/2.0);
        temp_plane.normal = Vec3(0.0, 0.0, -1.0);
        temp_plane.width = self.size.x();
        temp_plane.height = self.size.y();
        temp_plane_hit = temp_plane.hit(&local_ray);
        if temp_plane_hit.is_some() {
            assign_plane_if_some(temp_plane, return_plane);
            return temp_plane_hit;
        }

        // Hit left side?
        temp_plane.origin = Vec3(-self.size.x()/2.0, 0.0, 0.0);
        temp_plane.normal = Vec3(-1.0, 0.0, 0.0);
        temp_plane.width = self.size.z();
        temp_plane.height = self.size.y();
        temp_plane_hit = temp_plane.hit(&local_ray);
        if temp_plane_hit.is_some() {
            assign_plane_if_some(temp_plane, return_plane);
            return temp_plane_hit;
        }

        // Hit right side?
        temp_plane.origin = Vec3(self.size.x()/2.0, 0.0, 0.0);
        temp_plane.normal = Vec3(1.0, 0.0, 0.0);
        temp_plane.width = self.size.z();
        temp_plane.height = self.size.y();
        temp_plane_hit = temp_plane.hit(&local_ray);
        if temp_plane_hit.is_some() {
            assign_plane_if_some(temp_plane, return_plane);
            return temp_plane_hit;
        }
        
        // Hit top?
        temp_plane.origin = Vec3(0.0, self.size.y()/2.0, 0.0);
        temp_plane.normal = Vec3(0.0, 1.0, 0.0);
        temp_plane.width = self.size.x();
        temp_plane.height = self.size.z();
        temp_plane_hit = temp_plane.hit(&local_ray);
        if temp_plane_hit.is_some() {
            assign_plane_if_some(temp_plane, return_plane);
            return temp_plane_hit;
        }
        
        // Hit bottom?
        temp_plane.origin = Vec3(0.0, -self.size.y()/2.0, 0.0);
        temp_plane.normal = Vec3(0.0, -1.0, 0.0);
        temp_plane.width = self.size.x();
        temp_plane.height = self.size.z();
        temp_plane_hit = temp_plane.hit(&local_ray);
        if temp_plane_hit.is_some() {
            assign_plane_if_some(temp_plane, return_plane);
            return temp_plane_hit;
        }

        return None;
    }
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

        // Get the plane that the ray intersects with.
        match self.local_plane_hit(&local_ray, None) {
            Some(mut rec) => {
                rec.material = self.material.clone();
                rec.p = r.point_at_paramter(rec.t);
                return Some(rec);
            },
            None => {return None;}
        }
    }
}
