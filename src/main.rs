use std::fs::File;
use std::io::BufWriter;
use png::HasParameters;
use std::sync::Arc;
use rand::prelude::*;

use rayon::prelude::*;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

mod camera;
use camera::Camera;

mod hitable;
use hitable::*;

mod material;
use material::*;

mod sphere;
use sphere::Sphere;

fn color(r: &Ray, world: &HitList, depth: usize) -> Vec3 {
    match world.hit(r, 0.001, std::f64::MAX) {
        Some(rec) => {
            let mut scattered = Ray::NONE;
            let mut attenuation = Vec3::ZEROS;
            let mut col = Vec3::ZEROS;
            if depth < 50 && rec.material.scatter(&r, &rec, &mut attenuation, &mut scattered) {
                col = attenuation*color(&scattered, &world, depth+1);
            }
            return col;
        }, 
        None => {
            let unit_direction : Vec3 = r.direction.make_unit_vector();
            let t: f64 = 0.5*(unit_direction.y() + 1.0);
            return (1.0-t)*Vec3::ONES + t*Vec3::new(0.5, 0.7, 1.0);
        }
    }
}

fn make_random_scene() -> HitList {
    let n = 500;
    let mut list: HitList = Vec::new();
    list.push(Box::new(Sphere{
            center: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Arc::new( material::Lambertian{ albedo: 0.5*Vec3::ONES } ),
        }));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Vec3::new(
                (a as f64)+0.9*rand::random::<f64>(),
                0.2,
                (b as f64)+0.9*rand::random::<f64>()
            );
            if (center-Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if (choose_mat < 0.8) {
                    let rand_col = Vec3::new(rand::random::<f64>()*rand::random::<f64>(), rand::random::<f64>()*rand::random::<f64>(), rand::random::<f64>()*rand::random::<f64>());
                    list.push(Box::new(Sphere{
                        center,
                        radius: 0.2,
                        material: Arc::new( material::Lambertian{ albedo: rand_col }),
                    }));
                } else if choose_mat < 0.95 {
                    let rand_col = 0.5*Vec3::new(1.0 + rand::random::<f64>(), 1.0 + rand::random::<f64>(), 1.0 + rand::random::<f64>());
                    list.push(Box::new(Sphere{
                        center,
                        radius: 0.2,
                        material: Arc::new( material::Metal{ albedo: rand_col, fuzz: 0.5*rand::random::<f64>() } ),
                    }));
                } else {
                    list.push(Box::new(Sphere{
                        center,
                        radius: 0.2,
                        material: Arc::new( material::Dielectric{ ref_idx: 1.5 }),
                    }));
                }
            }
        }
    }

    list.push(Box::new(Sphere{
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new( material::Dielectric{ ref_idx: 1.5 } ),
    }));
    
    list.push(Box::new(Sphere{
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new( material::Lambertian{ albedo: Vec3::new(0.4, 0.2, 0.1) } ),
    }));
    
    list.push(Box::new(Sphere{
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new( material::Metal{ albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0 } ),
    }));

    return list;
}

fn main() -> std::io::Result<()>{

    let world = make_random_scene();

    let file = File::create("./out.png")?;
    let ref mut w = BufWriter::new(file);

    let nx: u32 = 1200;
    let ny: u32 = 800;
    let nparts: u32 = 36;
    let ns_per_part: u32 = 2;
    
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

    let mut encoder = png::Encoder::new(w, nx, ny);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    println!("Initializing temporary buffers...");
    let mut buffer_array: Vec<Vec<f64>> = Vec::new();
    for i in 0..nparts {
        buffer_array.push(vec![0.0 as f64; (nx*ny*4) as usize]);
    }
    println!("done...");
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
            print!(".");
        }
    );

    println!("Averaging...");
    let mut final_buffer = vec![0 as u8; (nx*ny*4) as usize];
    for i in 0..(nx*ny*4) {
        let mut pixel_value: f64 = 0.0;
        for buffer in buffer_array.iter() {
            pixel_value += buffer[i as usize] / (nparts as f64);
        }
        final_buffer[i as usize] = (255.99*pixel_value) as u8;
    }

    writer.write_image_data(&final_buffer)?;
    println!("Done.");
    Ok(())
}
