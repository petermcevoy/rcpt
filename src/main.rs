use std::fs::File;
use std::io::BufWriter;
use png::HasParameters;
use std::rc::Rc;

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

fn main() -> std::io::Result<()>{

    // Make objects
    let lamb = Rc::new(material::Lambertian{albedo: 0.6*Vec3::ONES});
    let s1 = Sphere{center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5, material: lamb.clone()};
    let s2 = Sphere{center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0, material: lamb.clone()};
    let mut world : HitList = Vec::new();
    world.push(&s1);
    world.push(&s2);


    let file = File::create("./out.png")?;
    let ref mut w = BufWriter::new(file);

    let nx: u32 = 200;
    let ny: u32 = 100;
    let ns: u32 = 100;
    
    let camera = Camera::DEFAULT;

    let mut encoder = png::Encoder::new(w, nx, ny);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    let mut buffer: Vec<u8> = vec![0; (nx*ny*4) as usize];
    for y in 0..ny {
        for x in 0..nx {
            let mut col = Vec3::ZEROS;
            for _s in 0..ns {
                let u = (x as f64 + rand::random::<f64>()) / (nx as f64);
                let v = (y as f64 + rand::random::<f64>()) / (ny as f64);
                let r = camera.get_ray(u, v);
                
                col += color(&r, &world, 0);
            }
            col /= ns as f64;
            col = Vec3::new( col.r().sqrt(), col.g().sqrt(), col.b().sqrt() );
            buffer[(((ny-1-y)*nx + x)*4 + 0) as usize] = (255.99*col.r()) as u8;
            buffer[(((ny-1-y)*nx + x)*4 + 1) as usize] = (255.99*col.g()) as u8;
            buffer[(((ny-1-y)*nx + x)*4 + 2) as usize] = (255.99*col.b()) as u8;
            buffer[(((ny-1-y)*nx + x)*4 + 3) as usize] = 255;
        }
    }

    writer.write_image_data(&buffer)?;
    println!("Done.");
    Ok(())
}
