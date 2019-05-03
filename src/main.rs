use std::fs::File;
use std::io::BufWriter;
use png::HasParameters;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

mod hitable;
use hitable::*;

mod sphere;
use sphere::Sphere;

//fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> f64 {
//    let oc = r.origin - center;
//    let a = Vec3::dot(r.direction, r.direction);
//    let b = 2.0 * Vec3::dot(oc, r.direction);
//    let c = Vec3::dot(oc, oc) - radius*radius;
//    let discriminant = b*b - 4.0*a*c;
//    if discriminant < 0.0 {
//        return -1.0;
//    } else {
//        return (-b - discriminant.sqrt()) / (2.0*a);
//    }
//}

fn color(r: &Ray, world: &Vec<&Hitable>) -> Vec3 {
    let mut rec = HitRecord::NONE;
    if world.hit(r, 0.0, std::f64::MAX, &mut rec) {
        return 0.5*Vec3::new(rec.normal.x()+1.0, rec.normal.y()+1.0, rec.normal.z()+1.0);
    } else {
        let unit_direction : Vec3 = r.direction.make_unit_vector();
        let t: f64 = 0.5*(unit_direction.y() + 1.0);
        return (1.0-t)*Vec3::ONES + t*Vec3::new(0.5, 0.7, 1.0);
    }
}

fn main() -> std::io::Result<()>{

    // Make objects
    let s1 = Sphere{center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5};
    let s2 = Sphere{center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0};
    let mut world : HitList = Vec::new();
    world.push(&s1);
    world.push(&s2);


    let file = File::create("./out.png")?;
    let ref mut w = BufWriter::new(file);

    let nx: u32 = 200;
    let ny: u32 = 100;
    
    let mut encoder = png::Encoder::new(w, nx, ny);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    //let mut buffer = Vec::<u8>::with_capacity((nx*ny*4) as usize);
    let mut buffer: Vec<u8> = vec![0; (nx*ny*4) as usize];
    for y in 0..ny {
        for x in 0..nx {
            // TODO....
            let u = (x as f64) / (nx as f64);
            let v = (y as f64) / (ny as f64);
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);

            let col = color(&r, &world);
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
