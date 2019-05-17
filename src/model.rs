use crate::{
    Vec3,
    Quaternion,
    ray::Ray,
    hitable::{Hit, Hitable, T_MIN, T_MAX},
    materials::Material,
    aabb::AABB,
    ray::random_to_sphere,
    PI,
    ray::UVW
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

    fn pdf_value(&self, ray_origin: Vec3, v: Vec3) -> f64 {
        match self.hit(&Ray::new(ray_origin, v)) {
            Some(rec) => {
                let cos_theta_max = (1.0 - self.radius * self.radius / (self.center - ray_origin).squared_length()).sqrt();
                let solid_angle = 2.0*PI*(1.0 - cos_theta_max);

                1.0 / solid_angle
            },
            None => {return 0.0;},
        }
    }
    fn random(&self, ray_origin: Vec3) -> Vec3 {
        let direction = self.center - ray_origin;
        let distance_squared = direction.squared_length();

        let onb = UVW::onb_from_w(direction);
        let tmp = random_to_sphere(self.radius, distance_squared);
        //println!("{:?}", distance_squared);
        onb.local(tmp)
    }
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
        let irot = Quaternion::rot_from_vecs(self.normal.make_unit_vector(), local_normal);
        let local_ray = Ray{
            origin: irot.transform_vec(r.origin - self.origin),
            direction: irot.transform_vec(r.direction),
        };
        
        // Check if we intersect the infinite plane.
        let denom = local_normal.dot(local_ray.direction);
        let t = (-1.0*local_ray.origin).dot(local_normal) / denom;

        if t > 1e-5 {
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
        None
    }

    fn pdf_value(&self, ray_origin: Vec3, v: Vec3) -> f64 {
        match self.hit(&Ray::new(ray_origin, v)) {
            Some(rec) => {
                let area = (self.width*self.height);
                let distance_squared = rec.t * rec.t * v.squared_length();
                let cosine = (v.dot(rec.normal)).abs() / v.length();
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
		
        let local_normal = Vec3::new(0.0, 0.0, 1.0);
        let rot = Quaternion::rot_from_vecs(local_normal, self.normal);
        let mut tmp = rot.transform_vec(local_random_point);
        let global_random_point = tmp + self.origin;

        return global_random_point - ray_origin;
    }
}

pub struct Cuboid {
    pub origin: Vec3,
    pub rot: Quaternion,
    size: Vec3,
    pub material: Option<Arc<Material + Send>>,
    planes_cache: Vec<Box<Hitable>>
}

impl Cuboid {
    pub fn new() -> Cuboid {
        let mut c = Cuboid{
            origin: Vec3::ZEROS,
            rot: Quaternion::UNIT,
            size: Vec3::ONES,
            material: None,
            planes_cache: Vec::<Box<Hitable>>::new()
        };
        c.generate_planes_cache();
        c
    }
    pub fn origin(mut self, origin: Vec3) -> Self {
        self.origin = origin;
        self
    }
    pub fn rot(mut self, rot: Quaternion) -> Self {
        self.rot = rot;
        self
    }
    pub fn size(mut self, size: Vec3) -> Self {
        self.size = size;
        self.generate_planes_cache();
        self
    }
    pub fn material(mut self, material: Arc<Material + Send>) -> Self {
        self.material = Some(material);
        self
    }
    pub fn build(self) -> Cuboid {
        self
    }

    fn generate_planes_cache(&mut self) -> &mut Self {
        self.planes_cache = vec![
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

        self
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
        match self.planes_cache.hit(&local_ray) {
            Some(mut rec) => {
                rec.material = self.material.clone();
                rec.p = r.point_at_paramter(rec.t);
                rec.normal = self.rot.transform_vec(rec.normal);
                return Some(rec);
            },
            None => {return None;}
        }
    }
}
