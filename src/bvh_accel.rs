use crate::core::*;

enum BVHSplitMethod {
    SAH
}

pub struct BVHAccel<'a> {
    max_prims_in_node: usize,
    split_method: BVHSplitMethod,
    shapes: Vec<&'a dyn Shape>,
}

struct BVHPrimitiveInfo {
   pub primitive_number: usize,
   pub bounds: Bounds3,
   pub centroid: Vec3
}

impl <'a> BVHAccel<'a> {
    fn new(max_prims_in_node: usize, split_method: BVHSplitMethod, shapes: Vec<& dyn Shape>) -> BVHAccel {
        BVHAccel {
            max_prims_in_node,
            split_method,
            shapes
        }
    }

    fn build(&self) {
        // initialize primite info array
        let info_array: Vec<BVHPrimitiveInfo> = Vec::with_capacity(self.shapes.len());
        for (i, shape) in self.shapes.iter().enumerate() {
            info_array.push(
                BVHAccel{
                    primitive_number: i, 
                    bounds: shape.world_bounds(),
                    centroid: // TODO
                }
            );
        }

        // TODO: Build BVH tree using info array.
        
        // TODO: Compute representation for depth first traversal of tree.
    }
}
