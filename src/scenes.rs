use crate::*;
use crate::core::*;


pub fn make_cornell(camera: &mut Camera) -> Vec<Box<Hitable>> {

    {
        let lookfrom = Vec3::new(278.0, 278.0, -800.0);
        let lookat = Vec3::new(278.0, 278.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let fov = 40.0;
        let aspect = (NX as Real)/(NY as Real);
        let aperture = 0.0;
        let focus_dist = 10.0;//(lookfrom-lookat).length();
        *camera = Camera::new(lookfrom, lookat, up, fov, aspect, aperture, focus_dist);
    }
    
    // Make spectrums
    let spectrum_light_lambdas  = vec![ 400.0,  500.0,  600.0,  700.0 ];
    let spectrum_light_values   = vec![ 0.0,    8.0,    15.6,   18.4 ];
    print!("--> Sampled Light:  ");
    let sampled_light = spectrum::Spectrum::from_sampled(
        &spectrum_light_lambdas, 
        &spectrum_light_values,
        4);

    let spectrum_refl_lambdas = vec![ 400.0, 404.0, 408.0, 412.0, 416.0, 420.0, 424.0, 428.0, 432.0, 436.0, 440.0, 444.0, 448.0, 452.0, 456.0, 460.0, 464.0, 468.0, 472.0, 476.0, 480.0, 484.0, 488.0, 492.0, 496.0, 500.0, 504.0, 508.0, 512.0, 516.0, 520.0, 524.0, 528.0, 532.0, 536.0, 540.0, 544.0, 548.0, 552.0, 556.0, 560.0, 564.0, 568.0, 572.0, 576.0, 580.0, 584.0, 588.0, 592.0, 596.0, 600.0, 604.0, 608.0, 612.0, 616.0, 620.0, 624.0, 628.0, 632.0, 636.0, 640.0, 644.0, 648.0, 652.0, 656.0, 660.0, 664.0, 668.0, 672.0, 676.0, 680.0, 684.0, 688.0, 692.0, 696.0, 700.0 ];

    let spectrum_refl_white = vec![ 0.343, 0.445, 0.551, 0.624, 0.665, 0.687, 0.708, 0.723, 0.715, 0.710, 0.745, 0.758, 0.739, 0.767, 0.777, 0.765, 0.751, 0.745, 0.748, 0.729, 0.745, 0.757, 0.753, 0.750, 0.746, 0.747, 0.735, 0.732, 0.739, 0.734, 0.725, 0.721, 0.733, 0.725, 0.732, 0.743, 0.744, 0.748, 0.728, 0.716, 0.733, 0.726, 0.713, 0.740, 0.754, 0.764, 0.752, 0.736, 0.734, 0.741, 0.740, 0.732, 0.745, 0.755, 0.751, 0.744, 0.731, 0.733, 0.744, 0.731, 0.712, 0.708, 0.729, 0.730, 0.727, 0.707, 0.703, 0.729, 0.750, 0.760, 0.751, 0.739, 0.724, 0.730, 0.740, 0.737 ];
    let sampled_refl_white = spectrum::Spectrum::from_sampled(
        &spectrum_refl_lambdas, 
        &spectrum_refl_white,
        75);

    let spectrum_refl_red = vec![ 0.040, 0.046, 0.048, 0.053, 0.049, 0.050, 0.053, 0.055, 0.057, 0.056, 0.059, 0.057, 0.061, 0.061, 0.060, 0.062, 0.062, 0.062, 0.061, 0.062, 0.060, 0.059, 0.057, 0.058, 0.058, 0.058, 0.056, 0.055, 0.056, 0.059, 0.057, 0.055, 0.059, 0.059, 0.058, 0.059, 0.061, 0.061, 0.063, 0.063, 0.067, 0.068, 0.072, 0.080, 0.090, 0.099, 0.124, 0.154, 0.192, 0.255, 0.287, 0.349, 0.402, 0.443, 0.487, 0.513, 0.558, 0.584, 0.620, 0.606, 0.609, 0.651, 0.612, 0.610, 0.650, 0.638, 0.627, 0.620, 0.630, 0.628, 0.642, 0.639, 0.657, 0.639, 0.635, 0.642 ];
    let sampled_refl_red = spectrum::Spectrum::from_sampled(
        &spectrum_refl_lambdas, 
        &spectrum_refl_red,
        75);

    let spectrum_refl_green = vec![ 0.092, 0.096, 0.098, 0.097, 0.098, 0.095, 0.095, 0.097, 0.095, 0.094, 0.097, 0.098, 0.096, 0.101, 0.103, 0.104, 0.107, 0.109, 0.112, 0.115, 0.125, 0.140, 0.160, 0.187, 0.229, 0.285, 0.343, 0.390, 0.435, 0.464, 0.472, 0.476, 0.481, 0.462, 0.447, 0.441, 0.426, 0.406, 0.373, 0.347, 0.337, 0.314, 0.285, 0.277, 0.266, 0.250, 0.230, 0.207, 0.186, 0.171, 0.160, 0.148, 0.141, 0.136, 0.130, 0.126, 0.123, 0.121, 0.122, 0.119, 0.114, 0.115, 0.117, 0.117, 0.118, 0.120, 0.122, 0.128, 0.132, 0.139, 0.144, 0.146, 0.150, 0.152, 0.157, 0.159 ];
    let sampled_refl_green = spectrum::Spectrum::from_sampled(
        &spectrum_refl_lambdas, 
        &spectrum_refl_green,
        75);

    let rgb_zero = spectrum::RGBSpectrum::new(0.0 as Real);
    let sampled_zero = spectrum::Spectrum::default();


    let scene: Vec<Box<Hitable>> = vec![
        Box::new( //Red
            Plane {
                origin: Vec3(555.0, 555.0/2.0, 555.0/2.0),
                normal: Vec3(-1.0, 0.0, 0.0),
                rot_around_normal: 0.0,
                width: 555.0,
                height: 555.0,
                material: Some(Arc::new( materials::Lambertian{ emit: sampled_zero, albedo: sampled_refl_red } ) )
            }  
        ),
        Box::new( //Green
            Plane {
                origin: Vec3(0.0, 555.0/2.0, 555.0/2.0),
                normal: Vec3(1.0, 0.0, 0.0),
                rot_around_normal: 0.0,
                width: 555.0,
                height: 555.0,
                material: Some(Arc::new( materials::Lambertian{ emit: sampled_zero, albedo: sampled_refl_green } ) )
            }  
        ),
        Box::new( //Light
            Plane {
                origin: Vec3(278.0, 554.0, 279.5),
                normal: Vec3(0.0, -1.0, 0.0),
                rot_around_normal: 0.0,
                width: 130.0,
                height: 105.0,
                material: Some(Arc::new( materials::Lambertian{ emit: sampled_light, albedo: sampled_zero } ) )
                //material: Some(Arc::new( materials::Lambertian{ emit: Vec3(0.0, 0.0, 0.0), albedo: Vec3::ZEROS } ) )
            }  
        ),
        Box::new( //White floor
            Plane {
                origin: Vec3(555.0/2.0, 0.0, 555.0/2.0),
                normal: Vec3(0.0, 1.0, 0.0),
                rot_around_normal: 0.0,
                width: 555.0,
                height: 555.0,
                material: Some(Arc::new( materials::Lambertian{ emit: sampled_zero, albedo: sampled_refl_white } ) )
            }  
        ),
        Box::new( //White ceiling
            Plane {
                origin: Vec3(555.0/2.0, 555.0, 555.0/2.0),
                normal: Vec3(0.0, -1.0, 0.0),
                rot_around_normal: 0.0,
                width: 555.0,
                height: 555.0,
                material: Some(Arc::new( materials::Lambertian{ emit: sampled_zero, albedo: sampled_refl_white } ) )
            }  
        ),
        Box::new( //White wall
            Plane {
                origin: Vec3(555.0/2.0, 555.0/2.0, 555.0),
                normal: Vec3(0.0, 0.0, -1.0),
                rot_around_normal: 0.0,
                width: 555.0,
                height: 555.0,
                material: Some(Arc::new( materials::Lambertian{ emit: sampled_zero, albedo: sampled_refl_white } ) )
            }  
        ),
        Box::new( //Small box
            Cuboid::new()
                .origin(Vec3(185.0, 165.0/2.0, 169.0))
                .size(Vec3(165.0, 165.0, 165.0))
                .rot(Quaternion::from_eulerangles(Vec3(0.0, -18.0*PI/180.0, 0.0)))
                //.material( Arc::new( materials::Lambertian{ emit: rgb_zero, albedo: rgb_refl_white } ) )
                .material( Arc::new( materials::Lambertian{ emit: sampled_zero, albedo: sampled_refl_white } ) )
                //.material( Arc::new( materials::Metal{ albedo: Vec3(0.73, 0.73, 0.73), fuzz: 0.0 } ) )
                .build()
        ),
        //Box::new( //Small sphere
        //    Sphere{
        //        center: Vec3(200.0, 165.0/2.0, 169.0),
        //        radius: 165.0/2.0,
        //        material: Some(Arc::new( materials::Lambertian{ emit: 2.0*Vec3::ONES, albedo: Vec3::ZEROS } ) )
        //        //material: Some(Arc::new( materials::Lambertian{ emit: Vec3(0.0, 0.0, 0.0), albedo: Vec3::ZEROS } ) )
        //    }
        //),
        Box::new( //Tall box
            Cuboid::new()
                .origin(Vec3(368.0, 330.0/2.0, 351.0))
                .size(Vec3(165.0, 330.0, 165.0))
                .rot(Quaternion::from_eulerangles(Vec3(0.0, 15.0*PI/180.0, 0.0)))
                //.material( Arc::new( materials::Lambertian{ emit: rgb_zero, albedo: rgb_refl_white } ) )
                .material( Arc::new( materials::Lambertian{ emit: sampled_zero, albedo: sampled_refl_white } ) )
                //.material( Arc::new( materials::Metal{ albedo: Vec3(0.73, 0.73, 0.73), fuzz: 0.0 } ) )
                .build()
        )
    ];
    return scene;
}

/*
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

    // Make spectrums
    let spectrum_light_lambdas  = vec![ 400.0,  500.0,  600.0,  700.0 ];
    let spectrum_light_values   = vec![ 0.0,    8.0,    15.6,   18.4 ];
    let rgb_light = spectrum::RGBSpectrum::from_sampled(
        &spectrum_light_lambdas, 
        &spectrum_light_values,
        4);

    let spectrum_refl_lambdas = vec![ 400.0, 404.0, 408.0, 412.0, 416.0, 420.0, 424.0, 428.0, 432.0, 436.0, 440.0, 444.0, 448.0, 452.0, 456.0, 460.0, 464.0, 468.0, 472.0, 476.0, 480.0, 484.0, 488.0, 492.0, 496.0, 500.0, 504.0, 508.0, 512.0, 516.0, 520.0, 524.0, 528.0, 532.0, 536.0, 540.0, 544.0, 548.0, 552.0, 556.0, 560.0, 564.0, 568.0, 572.0, 576.0, 580.0, 584.0, 588.0, 592.0, 596.0, 600.0, 604.0, 608.0, 612.0, 616.0, 620.0, 624.0, 628.0, 632.0, 636.0, 640.0, 644.0, 648.0, 652.0, 656.0, 660.0, 664.0, 668.0, 672.0, 676.0, 680.0, 684.0, 688.0, 692.0, 696.0, 700.0 ];

    let spectrum_refl_white = vec![ 0.343, 0.445, 0.551, 0.624, 0.665, 0.687, 0.708, 0.723, 0.715, 0.710, 0.745, 0.758, 0.739, 0.767, 0.777, 0.765, 0.751, 0.745, 0.748, 0.729, 0.745, 0.757, 0.753, 0.750, 0.746, 0.747, 0.735, 0.732, 0.739, 0.734, 0.725, 0.721, 0.733, 0.725, 0.732, 0.743, 0.744, 0.748, 0.728, 0.716, 0.733, 0.726, 0.713, 0.740, 0.754, 0.764, 0.752, 0.736, 0.734, 0.741, 0.740, 0.732, 0.745, 0.755, 0.751, 0.744, 0.731, 0.733, 0.744, 0.731, 0.712, 0.708, 0.729, 0.730, 0.727, 0.707, 0.703, 0.729, 0.750, 0.760, 0.751, 0.739, 0.724, 0.730, 0.740, 0.737 ];
    let rgb_refl_white = spectrum::RGBSpectrum::from_sampled(
        &spectrum_refl_lambdas, 
        &spectrum_refl_white,
        75);

    let spectrum_refl_red = vec![ 0.040, 0.046, 0.048, 0.053, 0.049, 0.050, 0.053, 0.055, 0.057, 0.056, 0.059, 0.057, 0.061, 0.061, 0.060, 0.062, 0.062, 0.062, 0.061, 0.062, 0.060, 0.059, 0.057, 0.058, 0.058, 0.058, 0.056, 0.055, 0.056, 0.059, 0.057, 0.055, 0.059, 0.059, 0.058, 0.059, 0.061, 0.061, 0.063, 0.063, 0.067, 0.068, 0.072, 0.080, 0.090, 0.099, 0.124, 0.154, 0.192, 0.255, 0.287, 0.349, 0.402, 0.443, 0.487, 0.513, 0.558, 0.584, 0.620, 0.606, 0.609, 0.651, 0.612, 0.610, 0.650, 0.638, 0.627, 0.620, 0.630, 0.628, 0.642, 0.639, 0.657, 0.639, 0.635, 0.642 ];
    let rgb_refl_red = spectrum::RGBSpectrum::from_sampled(
        &spectrum_refl_lambdas, 
        &spectrum_refl_red,
        75);

    let spectrum_refl_green = vec![ 0.092, 0.096, 0.098, 0.097, 0.098, 0.095, 0.095, 0.097, 0.095, 0.094, 0.097, 0.098, 0.096, 0.101, 0.103, 0.104, 0.107, 0.109, 0.112, 0.115, 0.125, 0.140, 0.160, 0.187, 0.229, 0.285, 0.343, 0.390, 0.435, 0.464, 0.472, 0.476, 0.481, 0.462, 0.447, 0.441, 0.426, 0.406, 0.373, 0.347, 0.337, 0.314, 0.285, 0.277, 0.266, 0.250, 0.230, 0.207, 0.186, 0.171, 0.160, 0.148, 0.141, 0.136, 0.130, 0.126, 0.123, 0.121, 0.122, 0.119, 0.114, 0.115, 0.117, 0.117, 0.118, 0.120, 0.122, 0.128, 0.132, 0.139, 0.144, 0.146, 0.150, 0.152, 0.157, 0.159 ];
    let rgb_refl_green = spectrum::RGBSpectrum::from_sampled(
        &spectrum_refl_lambdas, 
        &spectrum_refl_green,
        75);

    let list: Vec<Box<Hitable>> = vec![
        Box::new(Sphere{
            center: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Some(Arc::new( materials::Lambertian{ emit: spectrum::RGBSpectrum::new(0.0 as Real), albedo: rgb_refl_red } )),
        }),
        Box::new(Plane{
            origin: Vec3::new(0.0, 3.0, 0.0),
            normal: Vec3::new(0.0, -1.0, 0.0),
            rot_around_normal: 0.0,
            width: 4.0,
            height: 8.0,
            material: Some(Arc::new( materials::Lambertian{ emit: rgb_light, albedo: spectrum::RGBSpectrum::new(0.0 as Real) } )),
        }),
        //Box::new(Plane{
        //    origin: Vec3::new(0.0, 3.0, 0.0),
        //    normal: Vec3::new(0.0, -1.0, 0.0),
        //    rot_around_normal: 0.0,
        //    width: 4.0,
        //    height: 8.0,
        //    material: Some(Arc::new( materials::Lambertian{ emit: 2.0*Vec3::ONES, albedo: Vec3::ZEROS } )),
        //}),
        //Box::new(Sphere{
        //    center: Vec3::new(0.0, 1.0, 0.0),
        //    radius: 1.0,
        //    material: Some(Arc::new( materials::Lambertian{ emit: Vec3::ZEROS, albedo: 0.9*Vec3::ONES } )),
        //}),
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
*/

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

