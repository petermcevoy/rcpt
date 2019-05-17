use crate::{
    Vec3,
};

// Bridge between geometry processing and shading
pub trait Primitive {
    fn world_bound(&self) -> Bounds;    
    fn intersect(&self) -> ??;
}

// world_bound  - returns a box that encloses the primitive geometry in world space. 
// intersect    - Also responsible for initializing additional surface interaction. 
// area_light???
// get_material - returns pointer to materail instance assigned to primative. Can return null.
