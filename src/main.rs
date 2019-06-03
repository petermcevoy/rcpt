use std::sync::Arc;
use rand::prelude::*;
use rayon::prelude::*;

mod cgmath;
mod ray;
mod materials;
mod camera;
mod model;
mod aabb;
mod hitable;
mod scenes;
mod spectrum;
mod utils;

mod core {
    pub type Real = f64;
    pub const EPS: Real = 1e-5;
    pub const R_MAX: Real = std::f64::MAX;
    pub use crate::cgmath::{Vec3, Quaternion};
    pub use crate::spectrum::{RGBSpectrum};
    pub use crate::utils::*;
}
use cgmath::{Vec3, Quaternion};
use ray::{Ray, PDF, CosinePDF, HitablePDF, MixturePDF};
use hitable::{Hitable};
use camera::{Camera, random_in_unit_disk};
use materials::{Material};
use model::*;
use scenes::*;
use spectrum::*; //{RGBSpectrum, Spectrum, SampledSpectrum};
use crate::core::*;

use std::f64::consts::PI;
const light_shape: Plane = Plane {
    origin: Vec3(278.0, 554.0, 279.5),
    normal: Vec3(0.0, -1.0, 0.0),
    rot_around_normal: 0.0,
    width: 130.0,
    height: 105.0,
    material: None
};

fn color(r: &Ray, world: &Hitable, light: &dyn Hitable, depth: usize) -> RGBSpectrum {
    match world.hit(r) {
        Some(rec) => {
            let emitted;
            match rec.material.as_ref() {
                Some(mat) => {
                    emitted = mat.emitted(&r, &rec, rec.u, rec.v, rec.p);
                    if depth < 50 {
                        if let Some(srec) = mat.scatter(&r, &rec) {
                            if let Some(specular_ray) = srec.specular_ray {
                                return srec.attenuation * color(&specular_ray, world, light, depth+1);
                                //return color(&specular_ray, world, light, depth+1);
                            } else {
                                let hitable_pdf = HitablePDF::new(light, rec.p);
                                let mat_pdf = srec.pdf.unwrap();
                                let p = MixturePDF::new(&hitable_pdf, mat_pdf.as_ref());
                                //let p = CosinePDF::new(rec.normal);

                                let scattered = Ray{origin: rec.p, direction: p.generate()};
                                let pdf_val = p.value(scattered.direction);
                                if pdf_val == 0.0 { return emitted.clone(); }
                                let scattering_pdf_val = mat.scattering_pdf(&r, &rec, &scattered);

                                let val = emitted + srec.attenuation*scattering_pdf_val*color(&scattered, world, light, depth+1) / (pdf_val + 1e-5);
                                return val.clone();
                            }
                        }
                    }
                }, 
                None => {return spectrum::RGBSpectrum::new(0.0 as Real);}
            }
            return emitted.clone()
        }, 
        None => {return spectrum::RGBSpectrum::new(0.0 as Real);}
    }
}

const NX: usize = 250;
const NY: usize = 250;
const NPARTS: usize = 32;
const NS_PER_PART: usize = 16;

fn main() -> std::io::Result<()>{
    let mut camera = Camera::none();

    //let world = make_random_scene();
    //let world = make_dev_scene(&mut camera);
    let world = make_cornell(&mut camera);

    //Initializing temporary buffers for threads...
    let mut buffer_array = vec![vec![0.0; (NX*NY*4 as usize)]; NPARTS];
    
    // Dispatch threads.
    buffer_array.par_iter_mut().for_each(|buffer| {
        for y in 0..NY {
            for x in 0..NX {
                let mut col = Vec3::ZEROS;
                for _s in 0..NS_PER_PART {
                    let u = (x as f64 + rand::random::<f64>()) / (NX as f64);
                    let v = (y as f64 + rand::random::<f64>()) / (NY as f64);
                    let r = camera.get_ray(u, v);
                    
                    //col += color(&r, &world, world[2].as_ref(), 0);
                    //let gain = 0.1;
                    //let val = gain * integrate_spectrum_cie(&color(&r, &world, world[0].as_ref(), 0));
                    
                    let rgb = color(&r, &world, world[2].as_ref(), 0);
                    //let mut rgb: [Real; 3] = [0.0; 3];
                    //spectrum::xyz_to_rgb(&xyz.c, &mut rgb);

                    let val = Vec3::new(rgb[0], rgb[1], rgb[2]);
                    let val = Vec3::new(rgb[0], rgb[1], rgb[2]);
                    col += val;
                }
                col /= NS_PER_PART as f64;
                buffer[(((NY-1-y)*NX + x)*4 + 0) as usize] = col.r();
                buffer[(((NY-1-y)*NX + x)*4 + 1) as usize] = col.g();
                buffer[(((NY-1-y)*NX + x)*4 + 2) as usize] = col.b();
                buffer[(((NY-1-y)*NX + x)*4 + 3) as usize] = 1.0;
            }
        }
    });

    println!("Averaging...");
    let mut final_float_buffer = vec![0.0 as f64; (NX*NY*4) as usize];
    let mut final_buffer = vec![0 as u8; (NX*NY*4) as usize];
    for buffer in buffer_array.iter() {
        for i in 0..buffer.len() {
            let mut pixel_value = final_float_buffer[i] + buffer[i] / (NPARTS as f64);
            //if pixel_value > 1.0 { pixel_value = 1.0; }
            final_float_buffer[i] = pixel_value;
        }
    }

    //for i_x in 0..NX {
    //    for i_y in 0..NY {
    //        let offset = (i_y*NX + i_x)*4;
    //        let pixel_values = &final_float_buffer[offset..(offset + 4)];
    //        let pixel = Vec3::new(pixel_values[0], pixel_values[1], pixel_values[2]);
    //        final_float_buffer[offset + 0] = pixel.r();
    //        final_float_buffer[offset + 1] = pixel.g();
    //        final_float_buffer[offset + 2] = pixel.b();
    //        final_float_buffer[offset + 3] = 1.0;
    //    }
    //}
    let iter = final_buffer.iter_mut().zip(final_float_buffer.iter());
    for (final_pixel, float_pixel) in iter {
        let gamma_corrected = spectrum::gamma_correct(*float_pixel).min(1.0);
        *final_pixel = (255.99*gamma_corrected) as u8;
    }

    match lodepng::encode32_file("out2.png", &final_buffer, NX as usize, NY as usize) {
        Ok(()) => {},
        Err(err) => println!("Error writing file: {}", err),
    }
    
    println!("Done.");
    Ok(())
}
