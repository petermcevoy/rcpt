use std::sync::Arc;
use rand::prelude::*;
use rayon::prelude::*;

mod vec;
mod ray;
mod materials;
mod camera;
mod model;

use vec::Vec3;
use ray::Ray;
use camera::{Camera, random_in_unit_disk};
use materials::{Material};
use model::{Model, Sphere};

fn color(r: &Ray, world: &Model, depth: usize) -> Vec3 {
    match world.hit(r) {
        Some(rec) => {
            let mut scattered = Ray::NONE;
            let mut attenuation = Vec3::ZEROS;
            let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);
            if depth < 50 && rec.material.scatter(&r, &rec, &mut attenuation, &mut scattered) {
                return emitted + attenuation*color(&scattered, world, depth+1);
            } 
            return emitted
        }, 
        None => {
            return Vec3::ZEROS;
        }
    }
}

fn make_dev_scene() -> Vec<Box<Model>> {
    let mut spheres: Vec<Sphere> = vec![
        Sphere{
            center: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Arc::new( materials::Lambertian{ albedo: 0.8*Vec3::ONES } ),
        },
        Sphere{
            center: Vec3::new(2.5, 1.0, -2.0),
            radius: 1.0,
            material: Arc::new( materials::DiffuseLight{ albedo: 3.0*Vec3::ONES } ),
        },
        Sphere{
            center: Vec3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Arc::new( materials::Lambertian{ albedo: 0.9*Vec3::ONES } ),
        },
    ];
    let world: Vec<Box<Model>> = spheres.into_iter().map(|s| Box::new(s) as Box<Model>).collect();
    return world;
}
fn make_random_scene() -> Vec<Box<Model>> {
    let mut spheres: Vec<Sphere> = vec![
        Sphere{
            center: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Arc::new( materials::Lambertian{ albedo: 0.5*Vec3::ONES } ),
        },
        Sphere{
            center: Vec3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Arc::new( materials::Dielectric{ ref_idx: 1.5 } ),
        },
        Sphere{
            center: Vec3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: Arc::new( materials::Lambertian{ albedo: Vec3::new(0.4, 0.2, 0.1) } ),
        },
        Sphere{
            center: Vec3::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: Arc::new( materials::Metal{ albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0 } ),
        },
    ];

    fn random_material() -> Arc<Material + Send> {
        let rand_vec = Vec3::new(random(), random(), random());
        let rval = rand::random::<f64>();
        if rval < 0.7 {
            Arc::new( materials::Lambertian{ albedo: rand_vec})
        } else if rval < 0.9 {
            Arc::new(materials::Metal {
                albedo: Vec3::new(0.5, 0.5, 0.5) + 0.5*rand_vec, 
                fuzz: 0.5*rand::random::<f64>(),
            })
        } else {
            Arc::new( materials::Dielectric{ ref_idx: 1.5 })
        }
    }

    for _ in 0..500 {
        let r = 0.4 as f64;
        let Vec3(x, z, _) = random_in_unit_disk();
        let pos = 20.0 * Vec3(x, 0.0, z) + Vec3(0.0, r, 0.0);
        if spheres.iter().all(|s| (s.center - pos).length() >= s.radius + r) {
            spheres.push(Sphere {
                center: pos,
                radius: r,
                material: random_material(),
            });
        }
    }

    let world: Vec<Box<Model>> = spheres.into_iter().map(|s| Box::new(s) as Box<Model>).collect();
    return world;
}

fn main() -> std::io::Result<()>{

    let nx: u32 = 400;
    let ny: u32 = 200;
    let nparts: u32 = 12;
    let ns_per_part: u32 = 8;
    
    let camera;
    {
        let lookfrom = Vec3::new(13.0, 2.0, 3.0);
        let lookat = Vec3::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let fov = 20.0;
        let aspect = (nx as f64)/(ny as f64);
        let aperture = 0.3;
        let focus_dist = (lookfrom-lookat).length();
        camera = Camera::new(lookfrom, lookat, up, fov, aspect, aperture, focus_dist);
    }
    
    //let world = make_random_scene();
    let world = make_dev_scene();

    //Initializing temporary buffers for threads...
    let mut buffer_array: Vec<Vec<f64>> = Vec::new();
    for _ in 0..nparts {
        buffer_array.push(vec![0.0 as f64; (nx*ny*4) as usize]);
    }

    // Dispatch threads.
    buffer_array.par_iter_mut().for_each(|buffer| {
            for y in 0..ny {
                for x in 0..nx {
                    let mut col = Vec3::ZEROS;
                    for _s in 0..ns_per_part {
                        let u = (x as f64 + rand::random::<f64>()) / (nx as f64);
                        let v = (y as f64 + rand::random::<f64>()) / (ny as f64);
                        let r = camera.get_ray(u, v);
                        
                        col += color(&r, &world, 0);
                    }
                    col /= ns_per_part as f64;
                    col = Vec3::new( col.r().sqrt(), col.g().sqrt(), col.b().sqrt() );
                    buffer[(((ny-1-y)*nx + x)*4 + 0) as usize] = col.r();
                    buffer[(((ny-1-y)*nx + x)*4 + 1) as usize] = col.g();
                    buffer[(((ny-1-y)*nx + x)*4 + 2) as usize] = col.b();
                    buffer[(((ny-1-y)*nx + x)*4 + 3) as usize] = 1.0;
                }
            }
        }
    );

    println!("Averaging...");
    let mut final_buffer = vec![0 as u8; (nx*ny*4) as usize];
    for i in 0..(nx*ny*4) {
        let mut pixel_value: f64 = 0.0;
        for buffer in buffer_array.iter() {
            pixel_value += buffer[i as usize] / (nparts as f64);
        }
        if pixel_value > 1.0 { pixel_value = 1.0; }
        final_buffer[i as usize] = (255.99*pixel_value) as u8;
    }

    match lodepng::encode32_file("out2.png", &final_buffer, nx as usize, ny as usize) {
        Ok(()) => {},
        Err(err) => println!("Error writing file: {}", err),
    }
    
    println!("Done.");
    Ok(())
}
