mod core {
    #[cfg(not(feature = "real_is_double"))]
    pub type Real = f32;
    
    #[cfg(feature = "real_is_double")]
    pub type Real = f64;
    
    pub use std::f64::consts::PI as Real;

    pub use nalgebra as na;
    pub use na::{
        Vector3,
        Matrix3x4
    };

    pub type Vec3f = Vector3<Real>;
    pub type Mat34f = Matrix3x4<Real>;
}

//use crate::core::*;
mod primitive;
mod ray;

fn main() -> std::io::Result<()>{
    println!("hello!");
    Ok(())
}
