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

use cgmath::{Vec3, Quaternion};
use ray::Ray;
use hitable::{Hitable};
use camera::{Camera, random_in_unit_disk};
use materials::{Material};
use model::*;

use std::f64::consts::PI;

fn color(r: &Ray, world: &Hitable, depth: usize) -> Vec3 {
    match world.hit(r) {
        Some(rec) => {
            let mut scattered = Ray::NONE;
            let mut attenuation = Vec3::ZEROS;
            let emitted;
            match rec.material.as_ref() {
                Some(mat) => {
                    emitted = mat.emitted(rec.u, rec.v, &rec.p);
                    if depth < 10 && mat.scatter(&r, &rec, &mut attenuation, &mut scattered) {
                        return emitted + attenuation*color(&scattered, world, depth+1);
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
}

fn make_cornell() -> Vec<Box<Hitable>> {
    let scene: Vec<Box<Hitable>> = vec![
        Box::new( //Green
            Plane {
                origin: Vec3(555.0, 555.0/2.0, 555.0/2.0),
                normal: Vec3(-1.0, 0.0, 0.0),
                rot_around_normal: 0.0,
                width: 555.0,
                height: 555.0,
                material: Some(Arc::new( materials::Lambertian{ albedo: Vec3(0.12, 0.45, 0.15) } ) )
            }  
        ),
        Box::new( //Red
            Plane {
                origin: Vec3(0.0, 555.0/2.0, 555.0/2.0),
                normal: Vec3(1.0, 0.0, 0.0),
                rot_around_normal: 0.0,
                width: 555.0,
                height: 555.0,
                material: Some(Arc::new( materials::Lambertian{ albedo: Vec3(0.65, 0.05, 0.05) } ) )
            }  
        ),
        Box::new( //Light
            Plane {
                origin: Vec3((343.0 + 213.0)/2.0, 554.0, (332.0 + 227.0)/2.0),
                normal: Vec3(0.0, -1.0, 0.0),
                rot_around_normal: 0.0,
                width: (343.0 - 213.0),
                height: (332.0 - 227.0),
                material: Some(Arc::new( materials::DiffuseLight{ albedo: 2.0*Vec3(15.0, 15.0, 15.0) } ) )
            }  
        ),
        Box::new( //White floor
            Plane {
                origin: Vec3(555.0/2.0, 0.0, 555.0/2.0),
                normal: Vec3(0.0, 1.0, 0.0),
                rot_around_normal: 0.0,
                width: 555.0,
                height: 555.0,
                material: Some(Arc::new( materials::Lambertian{ albedo: Vec3(0.73, 0.73, 0.73) } ) )
            }  
        ),
        Box::new( //White ceiling
            Plane {
                origin: Vec3(555.0/2.0, 555.0, 555.0/2.0),
                normal: Vec3(0.0, -1.0, 0.0),
                rot_around_normal: 0.0,
                width: 555.0,
                height: 555.0,
                material: Some(Arc::new( materials::Lambertian{ albedo: Vec3(0.73, 0.73, 0.73) } ) )
            }  
        ),
        Box::new( //White wall
            Plane {
                origin: Vec3(555.0/2.0, 555.0/2.0, 555.0),
                normal: Vec3(0.0, 0.0, -1.0),
                rot_around_normal: 0.0,
                width: 555.0,
                height: 555.0,
                material: Some(Arc::new( materials::Lambertian{ albedo: Vec3(0.73, 0.73, 0.73) } ) )
            }  
        ),
        Box::new( //Small box 
            Cuboid {
                origin: Vec3(185.0, 165.0/2.0, 169.0),
                rot: Quaternion::from_eulerangles(Vec3(0.0, -18.0*PI/180.0, 0.0)),
                size: Vec3(165.0, 165.0, 165.0),
                material: Some(Arc::new( materials::Lambertian{ albedo: Vec3(0.73, 0.73, 0.73) } ) )
            }  
        ),
        Box::new( //Tall box 
            Cuboid {
                origin: Vec3(368.0, 330.0/2.0, 351.0),
                rot: Quaternion::from_eulerangles(Vec3(0.0, 15.0*PI/180.0, 0.0)),
                size: Vec3(165.0, 330.0, 165.0),
                material: Some(Arc::new( materials::Lambertian{ albedo: Vec3(0.73, 0.73, 0.73) } ) )
            }  
        ),
    ];
    return scene;
}
//fn make_dev_scene() -> Vec<Box<Hitable>> {
//    let list: Vec<Box<Hitable>> = vec![
//        Box::new(Sphere{
//            center: Vec3::new(0.0, -1000.0, 0.0),
//            radius: 1000.0,
//            material: Some(Arc::new( materials::Lambertian{ albedo: 0.8*Vec3::ONES } )),
//        }),
//        //Box::new(Sphere{
//        //    center: Vec3::new(2.5, 1.0, -2.0),
//        //    radius: 1.0,
//        //    material: Arc::new( materials::DiffuseLight{ albedo: 3.0*Vec3::ONES } ),
//        //}),
//        Box::new(Plane{
//            origin: Vec3::new(0.0, 3.0, -1.0),
//            normal: Vec3::new(0.0, -1.0, 1.0),
//            rot_around_normal: 0.0,
//            width: 8.0,
//            height: 3.0,
//            material: Some(Arc::new( materials::DiffuseLight{ albedo: 1.0*Vec3::ONES } )),
//        }),
//        Box::new(Sphere{
//            center: Vec3::new(0.0, 1.0, 0.0),
//            radius: 1.0,
//            material: Some(Arc::new( materials::Lambertian{ albedo: 0.9*Vec3::ONES } )),
//        }),
//        Box::new(Sphere{
//            center: Vec3::new(5.0, 1.0, -10.0),
//            radius: 1.0,
//            material: None,
//        }),
//        Box::new(Cuboid{
//            origin: Vec3(4.0, 1.0, 3.0),
//            rot: Quaternion::from_eulerangles(Vec3(0.0, PI/4.0, 0.0)),
//            size: Vec3(1.0, 2.0, 3.0),
//            material: None,
//        }),
//    ];
//    return list;
//}
//fn make_random_scene() -> Vec<Box<Hitable>> {
//    let mut spheres: Vec<Sphere> = vec![
//        Sphere{
//            center: Vec3::new(0.0, -1000.0, 0.0),
//            radius: 1000.0,
//            material: Arc::new( materials::Lambertian{ albedo: 0.5*Vec3::ONES } ),
//        },
//        Sphere{
//            center: Vec3::new(0.0, 1.0, 0.0),
//            radius: 1.0,
//            material: Arc::new( materials::Dielectric{ ref_idx: 1.5 } ),
//        },
//        Sphere{
//            center: Vec3::new(-4.0, 1.0, 0.0),
//            radius: 1.0,
//            material: Arc::new( materials::Lambertian{ albedo: Vec3::new(0.4, 0.2, 0.1) } ),
//        },
//        Sphere{
//            center: Vec3::new(4.0, 1.0, 0.0),
//            radius: 1.0,
//            material: Arc::new( materials::Metal{ albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0 } ),
//        },
//    ];
//
//    fn random_material() -> Arc<Material + Send> {
//        let rand_vec = Vec3::new(random(), random(), random());
//        let rval = rand::random::<f64>();
//        if rval < 0.7 {
//            Arc::new( materials::Lambertian{ albedo: rand_vec})
//        } else if rval < 0.9 {
//            Arc::new(materials::Metal {
//                albedo: Vec3::new(0.5, 0.5, 0.5) + 0.5*rand_vec, 
//                fuzz: 0.5*rand::random::<f64>(),
//            })
//        } else {
//            Arc::new( materials::Dielectric{ ref_idx: 1.5 })
//        }
//    }
//
//    for _ in 0..500 {
//        let r = 0.4 as f64;
//        let Vec3(x, z, _) = random_in_unit_disk();
//        let pos = 20.0 * Vec3(x, 0.0, z) + Vec3(0.0, r, 0.0);
//        if spheres.iter().all(|s| (s.center - pos).length() >= s.radius + r) {
//            spheres.push(Sphere {
//                center: pos,
//                radius: r,
//                material: random_material(),
//            });
//        }
//    }
//
//    let world: Vec<Box<Hitable>> = spheres.into_iter().map(|s| Box::new(s) as Box<Hitable>).collect();
//    return world;
//}

fn main() -> std::io::Result<()>{
    const nx: usize = 300;
    const ny: usize = 300;
    const nparts: usize = 24;
    const ns_per_part: usize = 8;
    
    //let camera;
    //{
    //    let lookfrom = Vec3::new(0.0, 2.0, 20.0);
    //    let lookat = Vec3::new(0.0, 1.0, 0.0);
    //    let up = Vec3::new(0.0, 1.0, 0.0);
    //    let fov = 20.0;
    //    let aspect = (nx as f64)/(ny as f64);
    //    let aperture = 0.3;
    //    let focus_dist = (lookfrom-lookat).length();
    //    camera = Camera::new(lookfrom, lookat, up, fov, aspect, aperture, focus_dist);
    //}
    // Camera for dev_scene and random_scene
    //let camera;
    //{
    //    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    //    let lookat = Vec3::new(0.0, 0.0, 0.0);
    //    let up = Vec3::new(0.0, 1.0, 0.0);
    //    let fov = 20.0;
    //    let aspect = (nx as f64)/(ny as f64);
    //    let aperture = 0.3;
    //    let focus_dist = (lookfrom-lookat).length();
    //    camera = Camera::new(lookfrom, lookat, up, fov, aspect, aperture, focus_dist);
    //}
    
    let camera;
    {
        let lookfrom = Vec3::new(278.0, 278.0, -800.0);
        let lookat = Vec3::new(278.0, 278.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let fov = 40.0;
        let aspect = (nx as f64)/(ny as f64);
        let aperture = 0.0;
        let focus_dist = 10.0;//(lookfrom-lookat).length();
        camera = Camera::new(lookfrom, lookat, up, fov, aspect, aperture, focus_dist);
    }
    
    //let world = make_random_scene();
    //let world = make_dev_scene();
    let world = make_cornell();

    //Initializing temporary buffers for threads...
    //let mut buffer_array: Vec<[f64; (nx*ny*4 as usize)]> = Vec::with_capacity(nparts);
    let mut buffer_array = vec![vec![0.0; (nx*ny*4 as usize)]; nparts];
    //let mut buffer_array: Vec<Vec<f64>> = Vec::new();
    //for _ in 0..nparts {
    //    buffer_array.push([0.0 as f64; (nx*ny*4)]);
    //}

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
    let mut final_float_buffer = vec![0.0 as f64; (nx*ny*4) as usize];
    let mut final_buffer = vec![0 as u8; (nx*ny*4) as usize];
    for buffer in buffer_array.iter() {
        for i in 0..buffer.len() {
            let mut pixel_value = final_float_buffer[i] + buffer[i] / (nparts as f64);
            if pixel_value > 1.0 { pixel_value = 1.0; }
            final_float_buffer[i] = pixel_value;
        }
    }

    let iter = final_buffer.iter_mut().zip(final_float_buffer.iter());
    for (final_pixel, float_pixel) in iter {
        *final_pixel = (255.99*float_pixel) as u8;
    }

    match lodepng::encode32_file("out2.png", &final_buffer, nx as usize, ny as usize) {
        Ok(()) => {},
        Err(err) => println!("Error writing file: {}", err),
    }
    
    println!("Done.");
    Ok(())
}
