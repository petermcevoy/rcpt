use crate::*;


pub fn make_cornell(camera: &mut Camera) -> Vec<Box<Hitable>> {

    {
        let lookfrom = Vec3::new(278.0, 278.0, -800.0);
        let lookat = Vec3::new(278.0, 278.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let fov = 40.0;
        let aspect = (NX as f64)/(NY as f64);
        let aperture = 0.0;
        let focus_dist = 10.0;//(lookfrom-lookat).length();
        *camera = Camera::new(lookfrom, lookat, up, fov, aspect, aperture, focus_dist);
    }

    let scene: Vec<Box<Hitable>> = vec![
        Box::new( //Green
            Plane {
                origin: Vec3(555.0, 555.0/2.0, 555.0/2.0),
                normal: Vec3(-1.0, 0.0, 0.0),
                rot_around_normal: 0.0,
                width: 555.0,
                height: 555.0,
                material: Some(Arc::new( materials::Lambertian{ emit: Vec3::ZEROS, albedo: Vec3(0.12, 0.45, 0.15) } ) )
            }  
        ),
        Box::new( //Red
            Plane {
                origin: Vec3(0.0, 555.0/2.0, 555.0/2.0),
                normal: Vec3(1.0, 0.0, 0.0),
                rot_around_normal: 0.0,
                width: 555.0,
                height: 555.0,
                material: Some(Arc::new( materials::Lambertian{ emit: Vec3::ZEROS, albedo: Vec3(0.65, 0.05, 0.05) } ) )
            }  
        ),
        Box::new( //Light
            Plane {
                origin: Vec3(278.0, 554.0, 279.5),
                normal: Vec3(0.0, -1.0, 0.0),
                rot_around_normal: 0.0,
                width: 130.0,
               height: 105.0,
                material: Some(Arc::new( materials::Lambertian{ emit: Vec3(15.0, 15.0, 15.0), albedo: Vec3::ZEROS } ) )
            }  
        ),
        Box::new( //White floor
            Plane {
                origin: Vec3(555.0/2.0, 0.0, 555.0/2.0),
                normal: Vec3(0.0, 1.0, 0.0),
                rot_around_normal: 0.0,
                width: 555.0,
                height: 555.0,
                material: Some(Arc::new( materials::Lambertian{ emit: Vec3::ZEROS, albedo: Vec3(0.73, 0.73, 0.73) } ) )
            }  
        ),
        Box::new( //White ceiling
            Plane {
                origin: Vec3(555.0/2.0, 555.0, 555.0/2.0),
                normal: Vec3(0.0, -1.0, 0.0),
                rot_around_normal: 0.0,
                width: 555.0,
                height: 555.0,
                material: Some(Arc::new( materials::Lambertian{ emit: Vec3::ZEROS, albedo: Vec3(0.73, 0.73, 0.73) } ) )
            }  
        ),
        Box::new( //White wall
            Plane {
                origin: Vec3(555.0/2.0, 555.0/2.0, 555.0),
                normal: Vec3(0.0, 0.0, -1.0),
                rot_around_normal: 0.0,
                width: 555.0,
                height: 555.0,
                material: Some(Arc::new( materials::Lambertian{ emit: Vec3::ZEROS, albedo: Vec3(0.73, 0.73, 0.73) } ) )
            }  
        ),
        Box::new( //Small box
            Cuboid::new()
                .origin(Vec3(185.0, 165.0/2.0, 169.0))
                .size(Vec3(165.0, 165.0, 165.0))
                .rot(Quaternion::from_eulerangles(Vec3(0.0, -18.0*PI/180.0, 0.0)))
                .material( Arc::new( materials::Lambertian{ emit: Vec3::ZEROS, albedo: Vec3(0.73, 0.73, 0.73) } ) )
                //.material( Arc::new( materials::Metal{ albedo: Vec3(0.73, 0.73, 0.73), fuzz: 0.0 } ) )
                .build()
        ),
        Box::new( //Tall box
            Cuboid::new()
                .origin(Vec3(368.0, 330.0/2.0, 351.0))
                .size(Vec3(165.0, 330.0, 165.0))
                .rot(Quaternion::from_eulerangles(Vec3(0.0, 15.0*PI/180.0, 0.0)))
                //.material( Arc::new( materials::Lambertian{ emit: Vec3::ZEROS, albedo: Vec3(0.73, 0.73, 0.73) } ) )
                .material( Arc::new( materials::Metal{ albedo: Vec3(0.73, 0.73, 0.73), fuzz: 0.0 } ) )
                .build()
        )
    ];
    return scene;
}
pub fn make_dev_scene(camera: &mut Camera) -> Vec<Box<Hitable>> {
    {
        let lookfrom = Vec3(0.0, 2.0, 25.0);
        let lookat = Vec3(0.0, 1.0, 0.0);
        let up = Vec3(0.0, 1.0, 0.0);
        let fov = 20.0;
        let aspect = (NX as f64)/(NY as f64);
        let aperture = 0.3;
        let focus_dist = (lookfrom-lookat).length();
        *camera = Camera::new(lookfrom, lookat, up, fov, aspect, aperture, focus_dist);
    }
    //{
    //    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    //    let lookat = Vec3::new(0.0, 0.0, 0.0);
    //    let up = Vec3::new(0.0, 1.0, 0.0);
    //    let fov = 20.0;
    //    let aspect = (NX as f64)/(NY as f64);
    //    let aperture = 0.3;
    //    let focus_dist = (lookfrom-lookat).length();
    //    *camera = Camera::new(lookfrom, lookat, up, fov, aspect, aperture, focus_dist);
    //}

    let list: Vec<Box<Hitable>> = vec![
        Box::new(Sphere{
            center: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Some(Arc::new( materials::Lambertian{ emit: Vec3::ZEROS, albedo: 0.8*Vec3::ONES } )),
        }),
        Box::new(Plane{
            origin: Vec3::new(0.0, 3.0, 0.0),
            normal: Vec3::new(0.0, -1.0, 0.0),
            rot_around_normal: 0.0,
            width: 4.0,
            height: 8.0,
            material: Some(Arc::new( materials::Lambertian{ emit: 2.0*Vec3::ONES, albedo: Vec3::ZEROS } )),
        }),
        Box::new(Sphere{
            center: Vec3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Some(Arc::new( materials::Lambertian{ emit: Vec3::ZEROS, albedo: 0.9*Vec3::ONES } )),
        }),
        //Box::new(Sphere{
        //    center: Vec3::new(5.0, 1.0, -10.0),
        //    radius: 1.0,
        //    material: None,
        //}),
        //Box::new(
        //    Cuboid::new()
        //    .origin(Vec3(4.0, 1.0, 3.0))
        //    .rot(Quaternion::from_eulerangles(Vec3(0.0, PI/4.0, 0.0)))
        //    .size(Vec3(1.0, 2.0, 3.0))
        //    .build()
        //),
    ];
    return list;
}

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

