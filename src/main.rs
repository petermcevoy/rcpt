use std::fs::File;
use std::io::prelude::*;
mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

fn color(r: &Ray) -> Vec3 {
    let unit_direction : Vec3 = r.dir.make_unit_vector();
    let t: f64 = 0.5*(unit_direction.y() + 1.0);
    return (1.0-t)*Vec3::ONES + t*Vec3::new(0.5, 0.7, 1.0);
}

fn main() -> std::io::Result<()>{
    let mut file = File::create("./out.pgm")?;

    let nx: i16 = 200;
    let ny: i16 = 100;

    write!(file, "P3\n")?;
    write!(file, "{} {}\n", nx, ny)?;
    write!(file, "255\n")?;
    
    let mut lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let mut horizontal = Vec3::new(4.0, 0.0, 0.0);
    let mut vertical = Vec3::new(0.0, 2.0, 0.0);
    let mut origin = Vec3::new(0.0, 0.0, 0.0);
    for y in (0..ny).rev() {
        for x in 0..nx {
            // TODO....
            let u = (x as f64) / (nx as f64);
            let v = (y as f64) / (ny as f64);
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
            let col = color(&r);

            let ir:i16 = (255.99*col.r()) as i16;
            let ig:i16 = (255.99*col.g()) as i16;
            let ib:i16 = (255.99*col.b()) as i16;

            write!(file, "{} {} {}\n", ir, ig, ib)?;
        }
    }

    println!("Done.");
    Ok(())
}
