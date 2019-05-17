use crate::core::*;
use crate::ray::*;

// Bridge between geometry processing and shading
pub trait Primitive {
    fn world_bound(&self) -> Bounds;    
    fn intersect(&self, ray: &mut Ray) -> Option<Hit>;
    fn get_material(&self) -> Option<&Material>;
}

// world_bound  - returns a box that encloses the primitive geometry in world space. 
// intersect    - Also responsible for initializing additional surface interaction. 
// area_light???
// get_material - returns pointer to materail instance assigned to primative. Can return null, if
// null ray intersections should be ignored; used for delianting volume for participating media.

//Have a pointer to the primitive that was hit in Hit Record.

pub struct TransformedPrimitive<'a> {
    tm: Matrix3x4<Real>,
    primitive: &'a dyn Primitive
}

//impl Primitive for TransofmredPrimitive {
//    fn intersect(&self) -> Option<Hit> {
//        // TODO
//        // transform the ray.
//        // call self.primitive.intersect(transformed_ray)
//    }
//    ...
//}

//pub struct Aggregate<'a> {
//    primitives: Vec<&'a dyn Primitive>
//}
//impl <'a> Primitive for Aggregate<'a> {
//    //raise error for get_material
//    //fn world_bound(&self) -> Bounds;    
//    //fn intersect(&self, ray: &mut Ray) -> Option<Hit>;
//    fn get_material(&self) -> Option<&Material> {
//        panic!("no!");
//        None
//    }
//}
