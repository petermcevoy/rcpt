use std::sync::Arc;
use rayon::prelude::*;

mod cgmath;
mod ray;
mod materials;
mod camera;
mod model;
mod hitable;
mod scenes;
mod bounds;
mod bvh_accel;

mod core {
    pub type Real = f32;

    pub use crate::cgmath::{Vec3, Vec3Axis, Quaternion};
    pub use crate::ray::{Ray, HitablePDF, MixturePDF, PDF};
    pub use crate::hitable::{Hitable, Hit};
    pub use crate::materials::{Material};
    pub use crate::bounds::{Bounds3};
    pub use crate::model::{Shape};
    pub use crate::bvh_accel::{BVHAccel, BVHSplitMethod};

    pub const PI: Real = std::f32::consts::PI as Real;
    pub const R_MAX: Real = std::f32::MAX;
    pub const EPS: Real = 1e-5;

    pub use rand::prelude::*;
}
    
use crate::core::*;
use camera::{Camera, random_in_unit_disk};
use model::*;
use scenes::{make_dev_scene, make_cornell};
use std::rc::Rc;

const light_shape: Plane = Plane {
    origin: Vec3(278.0, 554.0, 279.5),
    normal: Vec3(0.0, -1.0, 0.0),
    rot_around_normal: 0.0,
    width: 130.0,
    height: 105.0,
    material: None
};

fn color(r: &Ray, world: &Hitable, light: &dyn Hitable, depth: usize, bvh: &BVHAccel) -> Vec3 {

    match bvh.intersect(r) {
        Some(rec) => {
            let emitted;
            match rec.material.as_ref() {
                Some(mat) => {
                    emitted = mat.emitted(&r, &rec, rec.u, rec.v, rec.p);
                    if depth < 50 {
                        if let Some(srec) = mat.scatter(&r, &rec) {
                            if let Some(specular_ray) = srec.specular_ray {
                                return srec.attenuation * color(&specular_ray, world, light, depth+1, bvh);
                            } else {
                                //let hitable_pdf = HitablePDF::new(light, rec.p);
                                //let mat_pdf = srec.pdf.unwrap();
                                //let p = MixturePDF::new(&hitable_pdf, mat_pdf.as_ref());
                                let p = ray::CosinePDF::new(rec.normal);

                                let scattered = Ray::new(rec.p, p.generate());
                                let pdf_val = p.value(scattered.direction);
                                if pdf_val == 0.0 { return emitted; }
                                let scattering_pdf_val = mat.scattering_pdf(&r, &rec, &scattered);

                                let val = emitted + srec.attenuation*scattering_pdf_val*color(&scattered, world, light, depth+1, bvh) / (pdf_val + 1e-5);
                                return val;
                            }
                        }
                    }
                }, 
                None => {emitted = Vec3::ERROR;}
            }
            return emitted
        }, 
        None => {
            return Vec3::ZEROS;
        }
    }

   /* match world.hit(r) {
        Some(rec) => {
            let emitted;
            match rec.material.as_ref() {
                Some(mat) => {
                    emitted = mat.emitted(&r, &rec, rec.u, rec.v, rec.p);
                    if depth < 50 {
                        if let Some(srec) = mat.scatter(&r, &rec) {
                            if let Some(specular_ray) = srec.specular_ray {
                                return srec.attenuation * color(&specular_ray, world, light, depth+1);
                            } else {
                                let hitable_pdf = HitablePDF::new(light, rec.p);
                                let mat_pdf = srec.pdf.unwrap();
                                let p = MixturePDF::new(&hitable_pdf, mat_pdf.as_ref());

                                let scattered = Ray::new(rec.p, p.generate());
                                let pdf_val = p.value(scattered.direction);
                                if pdf_val == 0.0 { return emitted; }
                                let scattering_pdf_val = mat.scattering_pdf(&r, &rec, &scattered);

                                let val = emitted + srec.attenuation*scattering_pdf_val*color(&scattered, world, light, depth+1) / (pdf_val + 1e-5);
                                return val;
                            }
                        }
                    }
                }, 
                None => {emitted = Vec3::ERROR;}
            }
            return emitted
        }, 
        None => {
            return Vec3::ZEROS;
        }
    }*/
}

const NX: usize = 100;
const NY: usize = 100;
const NPARTS: usize = 1;
const NS_PER_PART: usize = 2;

fn main() -> std::io::Result<()>{
    let mut camera = Camera::none();

    //let world = make_random_scene();
    let world = make_dev_scene(&mut camera);
    //let world = make_cornell(&mut camera);

    println!("Building BVH...");
    let mut bvh;
    {
        let shapes: Vec<Rc<Shape>> = vec![
            Rc::new(Sphere{
                center: Vec3::new(0.0, -1000.0, 0.0),
                radius: 1000.0,
                material: Some(Arc::new( materials::Lambertian{ emit: Vec3::ZEROS, albedo: 0.9*Vec3::ONES } )),
            }),
            Rc::new(Sphere{
                center: Vec3::new(-2.0, 2.0, 0.0),
                radius: 1.0,
                material: Some(Arc::new( materials::Lambertian{ emit: 2.0*Vec3::ONES, albedo: Vec3::ZEROS } )),
            }),
            Rc::new(Sphere{
                center: Vec3::new(-2.5, 2.0, 0.0),
                radius: 0.1,
                material: Some(Arc::new( materials::Lambertian{ emit: Vec3::ZEROS, albedo: 0.9*Vec3::ONES } )),
            }),
        ];
        bvh = BVHAccel::new(10, BVHSplitMethod::Equal, shapes);
        bvh = bvh.build();
        println!("BVH finsihed.");
    }

    //Initializing temporary buffers for threads...
    let mut buffer_array = vec![vec![0.0; (NX*NY*4 as usize)]; NPARTS];

    // Dispatch threads.
    buffer_array.iter_mut().for_each(|buffer| {
        for y in 0..NY {
            for x in 0..NX {
                let mut col = Vec3::ZEROS;
                for _s in 0..NS_PER_PART {
                    let u = (x as Real + rand::random::<Real>()) / (NX as Real);
                    let v = (y as Real + rand::random::<Real>()) / (NY as Real);
                    let r = camera.get_ray(u, v);
                    
                    col += color(&r, &world, world[2].as_ref(), 0, &bvh);
                }
                col /= NS_PER_PART as Real;
                col = Vec3::new( col.r().sqrt(), col.g().sqrt(), col.b().sqrt() );
                buffer[(((NY-1-y)*NX + x)*4 + 0) as usize] = col.r();
                buffer[(((NY-1-y)*NX + x)*4 + 1) as usize] = col.g();
                buffer[(((NY-1-y)*NX + x)*4 + 2) as usize] = col.b();
                buffer[(((NY-1-y)*NX + x)*4 + 3) as usize] = 1.0;
            }
        }
    });

    println!("Averaging...");
    let mut final_float_buffer = vec![0.0 as Real; (NX*NY*4) as usize];
    let mut final_buffer = vec![0 as u8; (NX*NY*4) as usize];
    for buffer in buffer_array.iter() {
        for i in 0..buffer.len() {
            let mut pixel_value = final_float_buffer[i] + buffer[i] / (NPARTS as Real);
            if pixel_value > 1.0 { pixel_value = 1.0; }
            final_float_buffer[i] = pixel_value;
        }
    }

    let iter = final_buffer.iter_mut().zip(final_float_buffer.iter());
    for (final_pixel, float_pixel) in iter {
        *final_pixel = (255.99*float_pixel) as u8;
    }

    match lodepng::encode32_file("out2.png", &final_buffer, NX as usize, NY as usize) {
        Ok(()) => {},
        Err(err) => println!("Error writing file: {}", err),
    }
    
    println!("Done.");
    Ok(())
}
