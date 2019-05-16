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
        //if denom < 0.0 {
            let t = (-1.0*local_ray.origin).dot(local_normal) / denom;

			if t > 0.0 {
                // Check if we are in bounds.
                let local_p = local_ray.point_at_paramter(t);

                if (local_p.x()).abs() < self.width/2.0 && (local_p.y()).abs() < self.height/2.0 {
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
		//}
		None
    }

    fn pdf_value(&self, ray_origin: Vec3, v: Vec3) -> f64 {
        match self.hit(&Ray::new(ray_origin, v)) {
            Some(rec) => {
                let area = (self.width*self.height);
                //let distance_squared = rec.t * rec.t * v.squared_length();
                //let distance_squared = v.squared_length();
                let distance_squared = rec.t * rec.t * v.squared_length();
                let cosine = (v.dot(rec.normal)).abs() / v.length();
                return distance_squared / (cosine * area);
                //if cosine != 0.0 { distance_squared / (cosine * area) } else { 0.0 }
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
		
        let local_normal = Vec3::new(0.0, 0.0, 1.0);
        let rot = Quaternion::rot_from_vecs(local_normal, self.normal);
        let mut tmp = rot.transform_vec(local_random_point);
        tmp.0 = -1.0 * tmp.0;
        tmp.1 = -1.0 * tmp.1;
        let global_random_point = tmp + self.origin;

        return global_random_point - ray_origin;
    }
}

pub struct Cuboid {
    pub origin: Vec3,
    pub rot: Quaternion,
    pub size: Vec3,
    pub material: Option<Arc<Material + Send>>
}

//impl Cuboid {
//    //const PLANES: Vec<Plane> = vec![
//    //    Plane{ // FRONT
//    //        origin: Vec3(0.0, 0.0, 0.5),
//    //        normal: Vec3(0.0, 0.0, 1.0),
//    //        rot_around_normal: 0.0,
//    //        width: 1.0,
//    //        height: 1.0,
//    //        material: None
//    //    },
//    //    Plane{ // Back
//    //        origin: Vec3(0.0, 0.0, -0.5),
//    //        normal: Vec3(0.0, 0.0, -1.0),
//    //        rot_around_normal: 0.0,
//    //        width: 1.0,
//    //        height: 1.0,
//    //        material: None
//    //    },
//    //    Plane{ // Left side
//    //        origin: Vec3(-0.5, 0.0, 0),
//    //        normal: Vec3(-1.0, 0.0, 0.0),
//    //        rot_around_normal: 0.0,
//    //        width: 1.0,
//    //        height: 1.0,
//    //        material: None
//    //    },
//    //    Plane{ // Right side
//    //        origin: Vec3(0.5, 0.0, 0),
//    //        normal: Vec3(1.0, 0.0, 0.0),
//    //        rot_around_normal: 0.0,
//    //        width: 1.0,
//    //        height: 1.0,
//    //        material: None
//    //    },
//    //    Plane{ // Top
//    //        origin: Vec3(0.0, 0.5, 0),
//    //        normal: Vec3(0.0, 1.0, 0.0),
//    //        rot_around_normal: 0.0,
//    //        width: 1.0,
//    //        height: 1.0,
//    //        material: None
//    //    },
//    //    Plane{ // Bottom
//    //        origin: Vec3(0.0, -0.5, 0),
//    //        normal: Vec3(0.0, -1.0, 0.0),
//    //        rot_around_normal: 0.0,
//    //        width: 1.0,
//    //        height: 1.0,
//    //        material: None
//    //    },
//    //];
//
//    fn local_plane_hit(&self, local_ray: &Ray, return_plane: Option<&mut Plane>) -> Option<Hit> {
//    }
//}
impl Cuboid {
    fn local_plane_hit(&self, local_ray: &Ray) -> Option<Hit> {
        let planes: Vec<Box<Hitable>> = vec![
            Box::new(Plane{ // FRONT
                origin: Vec3(0.0, 0.0, self.size.z()/2.0),
                normal: Vec3(0.0, 0.0, 1.0),
                rot_around_normal: 0.0,
                width: self.size.x(),
                height: self.size.y(),
                material: None
            }),
            Box::new(Plane{ // Back
                origin: Vec3(0.0, 0.0, -self.size.z()/2.0),
                normal: Vec3(0.0, 0.0, -1.0),
                rot_around_normal: 0.0,
                width: self.size.x(),
                height: self.size.y(),
                material: None
            }),
            Box::new(Plane{ // Left side
                origin: Vec3(-self.size.x()/2.0, 0.0, 0.0),
                normal: Vec3(-1.0, 0.0, 0.0),
                rot_around_normal: 0.0,
                width: self.size.z(),
                height: self.size.y(),
                material: None
            }),
            Box::new(Plane{ // Right side
                origin: Vec3(self.size.x()/2.0, 0.0, 0.0),
                normal: Vec3(1.0, 0.0, 0.0),
                rot_around_normal: 0.0,
                width: self.size.z(),
                height: self.size.y(),
                material: None
            }),
            Box::new(Plane{ // Top
                origin: Vec3(0.0, self.size.y()/2.0, 0.0),
                normal: Vec3(0.0, 1.0, 0.0),
                rot_around_normal: 0.0,
                width: self.size.x(),
                height: self.size.z(),
                material: None
            }),
            Box::new( Plane{ // Bottom
                origin: Vec3(0.0, -self.size.y()/2.0, 0.0),
                normal: Vec3(0.0, -1.0, 0.0),
                rot_around_normal: 0.0,
                width: self.size.x(),
                height: self.size.z(),
                material: None
            }),
        ];

        //let planes_boxvec: Vec<Box<Hitable>> = planes.iter().map(|p| p as Box<Hitable>).collect();

        return planes.hit(local_ray);
    }
}

impl Hitable for Cuboid {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        //The local cuboid consits of 4 planes.
        //It has origin in (0,0,0) and size (1,1,1).
        let irot = self.rot.inv();
        let mut local_ray = Ray{
            origin: irot.transform_vec(r.origin - self.origin),
            direction: irot.transform_vec(r.direction),
        };
        
        // Get the plane that the ray intersects with.
        match self.local_plane_hit(&local_ray) {
            Some(mut rec) => {
                rec.material = self.material.clone();
                rec.p = r.point_at_paramter(rec.t);
                return Some(rec);
            },
            None => {return None;}
        }
    }
    //fn hit(&self, r: &Ray) -> Option<Hit> {
    //    // The local cuboid consits of 4 planes.
    //    // It has origin in (0,0,0) 
    //    
    //    let irot = self.rot.inv();
    //    let local_ray = Ray{
    //        origin: irot.transform_vec(r.origin - self.origin),
    //        direction: irot.transform_vec(r.direction),
    //    };

    //    // Get the plane that the ray intersects with.
    //    match self.local_plane_hit(&local_ray, None) {
    //        Some(mut rec) => {
    //            rec.material = self.material.clone();
    //            rec.p = r.point_at_paramter(rec.t);
    //            return Some(rec);
    //        },
    //        None => {return None;}
    //    }
    //}
}
