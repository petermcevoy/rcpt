use crate::*;
use crate::core::*;


pub fn make_colour_checker(camera: &mut Camera) -> Vec<Box<Hitable>> {
    {
        let lookfrom = Vec3::new(0.0, 0.0, 10.0);
        let lookat = Vec3::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let fov = 40.0;
        let aspect = (NX as Real)/(NY as Real);
        let aperture = 0.0;
        let focus_dist = 10.0;//(lookfrom-lookat).length();
        *camera = Camera::new(lookfrom, lookat, up, fov, aspect, aperture, focus_dist);
        camera.exposure = 0.01;
    }


    let spec_zero = spectrum::Spectrum::default();

    const N_PATCHES: usize = 24;
    let mut colour_cheker_patch_spec: [Spectrum; N_PATCHES] = [spectrum::Spectrum::default(); N_PATCHES];
    {
        const N_SAMPLES_PER_PATCH: usize = 36;
        const PATCH_LAMBDAS: [Real; N_SAMPLES_PER_PATCH] = [ 380.0, 390.0, 400.0, 410.0, 420.0, 430.0, 440.0, 450.0, 460.0, 470.0, 480.0, 490.0, 500.0, 510.0, 520.0, 530.0, 540.0, 550.0, 560.0, 570.0, 580.0, 590.0, 600.0, 610.0, 620.0, 630.0, 640.0, 650.0, 660.0, 670.0, 680.0, 690.0, 700.0, 710.0, 720.0, 730.0, ];
        const PATCH_VALUES: [[Real; N_SAMPLES_PER_PATCH]; N_PATCHES] = [
            // Dark Skin
            [ 0.055, 0.058, 0.061, 0.062, 0.062, 0.062, 0.062, 0.062, 0.062, 0.062, 0.062, 0.063, 0.065, 0.070, 0.076, 0.079, 0.081, 0.084, 0.091, 0.103, 0.119, 0.134, 0.143, 0.147, 0.151, 0.158, 0.168, 0.179, 0.188, 0.190, 0.186, 0.181, 0.182, 0.187, 0.196, 0.209 ],
            // Light Skin
            [ 0.117, 0.143, 0.175, 0.191, 0.196, 0.199, 0.204, 0.213, 0.228, 0.251, 0.280, 0.309, 0.329, 0.333, 0.315, 0.286, 0.273, 0.276, 0.277, 0.289, 0.339, 0.420, 0.488, 0.525, 0.546, 0.562, 0.578, 0.595, 0.612, 0.625, 0.638, 0.656, 0.678, 0.700, 0.717, 0.734 ],
            // Blue Sky
            [ 0.130, 0.177, 0.251, 0.306, 0.324, 0.330, 0.333, 0.331, 0.323, 0.311, 0.298, 0.285, 0.269, 0.250, 0.231, 0.214, 0.199, 0.185, 0.169, 0.157, 0.149, 0.145, 0.142, 0.141, 0.141, 0.141, 0.143, 0.147, 0.152, 0.154, 0.150, 0.144, 0.136, 0.132, 0.135, 0.147 ],
            // Foliage
            [ 0.051, 0.054, 0.056, 0.057, 0.058, 0.059, 0.060, 0.061, 0.062, 0.063, 0.065, 0.067, 0.075, 0.101, 0.145, 0.178, 0.184, 0.170, 0.149, 0.133, 0.122, 0.115, 0.109, 0.105, 0.104, 0.106, 0.109, 0.112, 0.114, 0.114, 0.112, 0.112, 0.115, 0.120, 0.125, 0.130 ],
            // Blue Flower
            [ 0.144, 0.198, 0.294, 0.375, 0.408, 0.421, 0.426, 0.426, 0.419, 0.403, 0.379, 0.346, 0.311, 0.281, 0.254, 0.229, 0.214, 0.208, 0.202, 0.194, 0.193, 0.200, 0.214, 0.230, 0.241, 0.254, 0.279, 0.313, 0.348, 0.366, 0.366, 0.359, 0.358, 0.365, 0.377, 0.398 ],
            // Bluish Green
            [ 0.136, 0.179, 0.247, 0.297, 0.320, 0.337, 0.355, 0.381, 0.419, 0.466, 0.510, 0.546, 0.567, 0.574, 0.569, 0.551, 0.524, 0.488, 0.445, 0.400, 0.350, 0.299, 0.252, 0.221, 0.204, 0.196, 0.191, 0.188, 0.191, 0.199, 0.212, 0.223, 0.232, 0.233, 0.229, 0.229 ],
            // Orange
            [ 0.054, 0.054, 0.053, 0.054, 0.054, 0.055, 0.055, 0.055, 0.056, 0.057, 0.058, 0.061, 0.068, 0.089, 0.125, 0.154, 0.174, 0.199, 0.248, 0.335, 0.444, 0.538, 0.587, 0.595, 0.591, 0.587, 0.584, 0.584, 0.590, 0.603, 0.620, 0.639, 0.655, 0.663, 0.663, 0.667 ],
            // Purplish Blue
            [ 0.122, 0.164, 0.229, 0.286, 0.327, 0.361, 0.388, 0.400, 0.392, 0.362, 0.316, 0.260, 0.209, 0.168, 0.138, 0.117, 0.104, 0.096, 0.090, 0.086, 0.084, 0.084, 0.084, 0.084, 0.084, 0.085, 0.090, 0.098, 0.109, 0.123, 0.143, 0.169, 0.205, 0.244, 0.287, 0.332 ],
            // Moderate Red
            [ 0.096, 0.115, 0.131, 0.135, 0.133, 0.132, 0.130, 0.128, 0.125, 0.120, 0.115, 0.110, 0.105, 0.100, 0.095, 0.093, 0.092, 0.093, 0.096, 0.108, 0.156, 0.265, 0.399, 0.500, 0.556, 0.579, 0.588, 0.591, 0.593, 0.594, 0.598, 0.602, 0.607, 0.609, 0.609, 0.610 ],
            // Purple
            [ 0.092, 0.116, 0.146, 0.169, 0.178, 0.173, 0.158, 0.139, 0.119, 0.101, 0.087, 0.075, 0.066, 0.060, 0.056, 0.053, 0.051, 0.051, 0.052, 0.052, 0.051, 0.052, 0.058, 0.073, 0.096, 0.119, 0.141, 0.166, 0.194, 0.227, 0.265, 0.309, 0.355, 0.396, 0.436, 0.478 ],
            // Yellow Green
            [ 0.061, 0.061, 0.062, 0.063, 0.064, 0.066, 0.069, 0.075, 0.085, 0.105, 0.139, 0.192, 0.271, 0.376, 0.476, 0.531, 0.549, 0.546, 0.528, 0.504, 0.471, 0.428, 0.381, 0.347, 0.327, 0.318, 0.312, 0.310, 0.314, 0.327, 0.345, 0.363, 0.376, 0.381, 0.378, 0.379 ],
            // Orange Yellow
            [ 0.063, 0.063, 0.063, 0.064, 0.064, 0.064, 0.065, 0.066, 0.067, 0.068, 0.071, 0.076, 0.087, 0.125, 0.206, 0.305, 0.383, 0.431, 0.469, 0.518, 0.568, 0.607, 0.628, 0.637, 0.640, 0.642, 0.645, 0.648, 0.651, 0.653, 0.657, 0.664, 0.673, 0.680, 0.684, 0.688 ],
            // Blue
            [ 0.066, 0.079, 0.102, 0.146, 0.200, 0.244, 0.282, 0.309, 0.308, 0.278, 0.231, 0.178, 0.130, 0.094, 0.070, 0.054, 0.046, 0.042, 0.039, 0.038, 0.038, 0.038, 0.038, 0.039, 0.039, 0.040, 0.041, 0.042, 0.044, 0.045, 0.046, 0.046, 0.048, 0.052, 0.057, 0.065 ],
            // Green
            [ 0.052, 0.053, 0.054, 0.055, 0.057, 0.059, 0.061, 0.066, 0.075, 0.093, 0.125, 0.178, 0.246, 0.307, 0.337, 0.334, 0.317, 0.293, 0.262, 0.230, 0.198, 0.165, 0.135, 0.115, 0.104, 0.098, 0.094, 0.092, 0.093, 0.097, 0.102, 0.108, 0.113, 0.115, 0.114, 0.114 ],
            // Red
            [ 0.050, 0.049, 0.048, 0.047, 0.047, 0.047, 0.047, 0.047, 0.046, 0.045, 0.044, 0.044, 0.045, 0.046, 0.047, 0.048, 0.049, 0.050, 0.054, 0.060, 0.072, 0.104, 0.178, 0.312, 0.467, 0.581, 0.644, 0.675, 0.690, 0.698, 0.706, 0.715, 0.724, 0.730, 0.734, 0.738 ],
            // Yellow
            [ 0.058, 0.054, 0.052, 0.052, 0.053, 0.054, 0.056, 0.059, 0.067, 0.081, 0.107, 0.152, 0.225, 0.336, 0.462, 0.559, 0.616, 0.650, 0.672, 0.694, 0.710, 0.723, 0.731, 0.739, 0.746, 0.752, 0.758, 0.764, 0.769, 0.771, 0.776, 0.782, 0.790, 0.796, 0.799, 0.804 ],
            // Magenta
            [ 0.145, 0.195, 0.283, 0.346, 0.362, 0.354, 0.334, 0.306, 0.276, 0.248, 0.218, 0.190, 0.168, 0.149, 0.127, 0.107, 0.100, 0.102, 0.104, 0.109, 0.137, 0.200, 0.290, 0.400, 0.516, 0.615, 0.687, 0.732, 0.760, 0.774, 0.783, 0.793, 0.803, 0.812, 0.817, 0.825 ],
            // Cyan
            [ 0.108, 0.141, 0.192, 0.236, 0.261, 0.286, 0.317, 0.353, 0.390, 0.426, 0.446, 0.444, 0.423, 0.385, 0.337, 0.283, 0.231, 0.185, 0.146, 0.118, 0.101, 0.090, 0.082, 0.076, 0.074, 0.073, 0.073, 0.074, 0.076, 0.077, 0.076, 0.075, 0.073, 0.072, 0.074, 0.079 ],
            // White 9.5 05D
            [ 0.189, 0.255, 0.423, 0.660, 0.811, 0.862, 0.877, 0.884, 0.891, 0.896, 0.899, 0.904, 0.907, 0.909, 0.911, 0.910, 0.911, 0.914, 0.913, 0.916, 0.915, 0.916, 0.914, 0.915, 0.918, 0.919, 0.921, 0.923, 0.924, 0.922, 0.922, 0.925, 0.927, 0.930, 0.930, 0.933 ],
            // Neutral 8 23D
            [ 0.171, 0.232, 0.365, 0.507, 0.567, 0.583, 0.588, 0.590, 0.591, 0.590, 0.588, 0.588, 0.589, 0.589, 0.591, 0.590, 0.590, 0.590, 0.589, 0.591, 0.590, 0.590, 0.587, 0.585, 0.583, 0.580, 0.578, 0.576, 0.574, 0.572, 0.571, 0.569, 0.568, 0.568, 0.566, 0.566 ],
            // Neutral 6.5 44D
            [ 0.144, 0.192, 0.272, 0.331, 0.350, 0.357, 0.361, 0.363, 0.363, 0.361, 0.359, 0.358, 0.358, 0.359, 0.360, 0.360, 0.361, 0.361, 0.360, 0.362, 0.362, 0.361, 0.359, 0.358, 0.355, 0.352, 0.350, 0.348, 0.345, 0.343, 0.340, 0.338, 0.335, 0.334, 0.332, 0.331 ],
            // Neutral 5 70D
            [ 0.105, 0.131, 0.163, 0.180, 0.186, 0.190, 0.193, 0.194, 0.194, 0.192, 0.191, 0.191, 0.191, 0.192, 0.192, 0.192, 0.192, 0.192, 0.192, 0.193, 0.192, 0.192, 0.191, 0.189, 0.188, 0.186, 0.184, 0.182, 0.181, 0.179, 0.178, 0.176, 0.174, 0.173, 0.172, 0.171 ],
            // Neutral 3.5_1 050D
            [ 0.068, 0.077, 0.084, 0.087, 0.089, 0.090, 0.092, 0.092, 0.091, 0.090, 0.090, 0.090, 0.090, 0.090, 0.090, 0.090, 0.090, 0.090, 0.090, 0.090, 0.090, 0.089, 0.089, 0.088, 0.087, 0.086, 0.086, 0.085, 0.084, 0.084, 0.083, 0.083, 0.082, 0.081, 0.081, 0.081 ],
            // Black 2_1 5D
            [ 0.031, 0.032, 0.032, 0.033, 0.033, 0.033, 0.033, 0.033, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.032, 0.033 ],
        ];

        for i in 0..N_PATCHES {
            colour_cheker_patch_spec[i] = spectrum::Spectrum::from_sampled(
                &PATCH_LAMBDAS,
                &PATCH_VALUES[i],
                N_SAMPLES_PER_PATCH);
        }
    }

    const N_PATCHES_PER_ROW: usize = 6;
    const N_PATCHES_PER_COL: usize = N_PATCHES / N_PATCHES_PER_ROW;

    let mut scene: Vec<Box<Hitable>> = Vec::with_capacity(N_PATCHES);
    for i in 0..N_PATCHES {
        let i_row = N_PATCHES_PER_COL - i / N_PATCHES_PER_ROW;
        let i_col = i % N_PATCHES_PER_ROW;

        const scale: Real = 0.8;
        let start_pos: Vec3 = Vec3::new(-((N_PATCHES_PER_ROW-1) as Real) / 2.0, -((N_PATCHES/N_PATCHES_PER_ROW - 1) as Real) / 2.0, 0.0) * scale ;
        let pos = start_pos +  Vec3(i_col as Real, i_row as Real, 0.0) * scale ;
        println!("{:?}", pos);
        scene.push(
            Box::new( //Light
                Plane {
                    origin: pos,
                    normal: Vec3(0.0, 0.0, -1.0),
                    rot_around_normal: 0.0,
                    width: 1.0 * scale,
                    height: 1.0 * scale,
                    material: Some(Arc::new( materials::Lambertian{ emit: spec_zero, albedo: colour_cheker_patch_spec[i] } ) )
                }  
            ) 
        );
    }

    return scene;
}
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
        camera.exposure = 1.0;
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

