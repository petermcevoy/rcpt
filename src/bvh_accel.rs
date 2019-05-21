use crate::core::*;
use std::rc::Rc;

pub enum BVHSplitMethod {
    Equal,
    Middle,
    SAH,
}

pub struct BVHAccel {
    max_prims_in_node: usize,
    split_method: BVHSplitMethod,
    shapes: Vec<Rc<Shape>>,
    nodes: Vec<BVHLinearNode>,
}

struct BVHPrimitiveInfo {
   pub primitive_number: usize,
   pub bounds: Bounds3,
   pub centroid: Vec3
}

#[derive(Copy, Clone)]
union BVHLinearNodeOffset {
    primitives_offset: usize,
    second_child_offset: usize,
}

#[derive(Copy, Clone)]
pub struct BVHLinearNode {
    bounds: Bounds3,
    offset: BVHLinearNodeOffset,
    n_primitives: usize,
    axis: Vec3Axis
}
impl std::fmt::Debug for BVHLinearNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        let offset_str;
        if self.n_primitives > 0 {
            offset_str = format!("( second_child_offset: {} )", unsafe{ self.offset.second_child_offset });
        } else {
            offset_str = format!("( primitives_offset: {} )", unsafe{ self.offset.primitives_offset });
        }

        write!(f, "BVHLinearNode (
               \t bounds: {:?},
               \t n_primitives: {},
               \t offset: {},
               \t axis: {:?},
               )\n", self.bounds, self.n_primitives, offset_str, self.axis)
    } 
}

#[derive(Debug)]
pub struct BVHBuildNode {
    bounds: Bounds3,
    children: [Option<Box<BVHBuildNode>>; 2],
    split_axis: Vec3Axis, 
    first_shape_offset: usize,
    n_shapes: usize
}
impl BVHBuildNode {
    fn init_leaf(first: usize, n: usize, bounds: Bounds3) -> BVHBuildNode {
        BVHBuildNode {
            first_shape_offset: first,
            n_shapes: n,
            bounds,
            children: [None, None],
            split_axis: Vec3Axis::X
        }
    }
    fn init_interior(axis: Vec3Axis, c1: Box<BVHBuildNode>, c2: Box<BVHBuildNode>) -> BVHBuildNode {
        BVHBuildNode {
            first_shape_offset: 0,
            n_shapes: 0,
            bounds: Bounds3::union(&c1.bounds, &c2.bounds),
            children: [Some(c1), Some(c2)],
            split_axis: axis
        }
    }
}
    
pub fn print_bvh(root: Box<BVHBuildNode>, depth: usize) {
    println!("===== Node {} =====", depth);
    println!("\t n_shapes: {}", root.n_shapes);
    println!("\t bounds: {:?}", root.bounds);
    println!("\t children: {:?}", root.children);
    
}

impl BVHAccel {
    pub fn new(max_prims_in_node: usize, split_method: BVHSplitMethod, shapes: Vec<Rc<Shape>>) -> BVHAccel {
        BVHAccel {
            max_prims_in_node,
            split_method,
            shapes,
            nodes: Vec::new()
        }
    }

    pub fn build(mut self) -> Self {
        // initialize primite info array
        let mut info_array: Vec<BVHPrimitiveInfo> = Vec::with_capacity(self.shapes.len());
        for (i, shape) in self.shapes.iter_mut().enumerate() {
            let shape_bounds = shape.world_bounds();
            info_array.push(
                BVHPrimitiveInfo{
                    primitive_number: i, 
                    bounds: shape_bounds,
                    centroid: 0.5*shape_bounds.pmin + 0.5 * shape_bounds.pmax
                }
            );
        }

        // TODO: Build BVH tree using info array.
        let mut total_nodes = 0;
        let mut root: Box<BVHBuildNode>;
        let mut ordered_prims_index: Vec<usize> = Vec::new();
        root = self.recursive_build(&mut info_array, 0, self.shapes.len(), 
                              &mut total_nodes, &mut ordered_prims_index);

        {
            let mut new_shapes: Vec<Rc<Shape>> = Vec::with_capacity(self.shapes.len());
            for i in 0..self.shapes.len() {
                new_shapes.push(self.shapes[ordered_prims_index[i]].clone());
            }
            self.shapes = new_shapes;
        }

        println!("BVH created with {} nodes for {} primatives", total_nodes, self.shapes.len());
        
        println!("Flattening BVH tree...");
        //self.nodes = Vec::with_capacity(total_nodes);
        let empty_lnode = BVHLinearNode {
            bounds: Bounds3::new(Vec3::ZEROS, Vec3::ZEROS),
            n_primitives: 0,
            offset: BVHLinearNodeOffset{ primitives_offset: 0 },
            axis: Vec3Axis::X
        };
        self.nodes = vec![empty_lnode; total_nodes];



        let mut offset = 0;
        self.flatten_tree(&Box::new(root), &mut offset);
        println!("Done.");

        println!("{:?}", self.nodes);
        
        return self

        // TODO: Compute representation for depth first traversal of tree.
    } 

    fn flatten_tree(&mut self, node: &BVHBuildNode, offset: &mut usize) -> usize {
        let mut lnode = BVHLinearNode {
            bounds: node.bounds,
            n_primitives: 0,
            offset: BVHLinearNodeOffset{ primitives_offset: 0 },
            axis: Vec3Axis::X
        };
        let old_offset = *offset;

        *offset += 1;

        if node.n_shapes > 0 {
            lnode.offset = BVHLinearNodeOffset{ primitives_offset: node.first_shape_offset};
            lnode.n_primitives = node.n_shapes;
        } else {
            lnode.axis = node.split_axis;
            lnode.n_primitives = 0;
            self.flatten_tree(node.children[0].as_ref().unwrap(), offset);
            lnode.offset.second_child_offset = self.flatten_tree(node.children[1].as_ref().unwrap(), offset);
        }
        
        self.nodes[old_offset] = lnode;
        return old_offset;
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        // Precompute common values.
        let inv_dir = Vec3::new(1.0 / ray.direction.x(), 1.0 / ray.direction.y(), 1.0 / ray.direction.z());
        let dir_is_neg: [bool; 3] = [inv_dir.x() < 0.0, inv_dir.y() < 0.0, inv_dir.z() < 0.0];

        // Follow ray though BVH
        let mut hit: Option<Hit> = None;
        let mut to_visit_offset = 0;
        let mut current_node_index = 0;
        let mut nodes_to_visit: [usize; 64] = [0; 64];
        loop {
            let lnode: &BVHLinearNode = &self.nodes[current_node_index];
            if lnode.bounds.intersect_p(ray, &R_MAX) {
                if lnode.n_primitives > 0 {
                    // Intersect with primatives in leaf node.
                    for i_prim in 0..lnode.n_primitives {
                        let tmp_hit = self.shapes[unsafe{lnode.offset.primitives_offset} + i_prim].intersect(ray);
                        if tmp_hit.is_some() {
                            hit = tmp_hit;
                        }
                    }
                    if (to_visit_offset == 0) {
                        break;
                    }
                    to_visit_offset -= 1;
                    current_node_index = nodes_to_visit[to_visit_offset];
                } else {
                    // Put far BVH node on nodes_to_visit
                    if dir_is_neg[lnode.axis.to_index()] {
                        nodes_to_visit[to_visit_offset] = current_node_index + 1;
                        to_visit_offset += 1;
                        current_node_index = unsafe{ lnode.offset.second_child_offset };
                    } else {
                        nodes_to_visit[to_visit_offset] = unsafe { lnode.offset.second_child_offset };
                        to_visit_offset += 1;
                        current_node_index += 1;
                    }
                }
            } else {
                if to_visit_offset == 0 { break; }
                to_visit_offset -= 1;
                current_node_index = nodes_to_visit[to_visit_offset];
            }
        }
        return hit;
    }


    fn recursive_build(&self, info_array: &mut Vec<BVHPrimitiveInfo>, start: usize, end: usize, 
                              total_nodes: &mut usize, ordered_prims: &mut Vec<usize>) -> Box<BVHBuildNode> {
        *total_nodes += 1;
        //let node = Box::new(BVHBuildNode::new()); // TODO

        // compute bounds for all nodes in BVH node
        let mut bounds = Bounds3::new(Vec3::ZEROS, Vec3::ZEROS);
        for i in start..end {
            bounds = Bounds3::union(&bounds, &info_array[i].bounds);
        }

        let n_shapes = end - start;
        if n_shapes == 1 { // Make Leaf Node
            let first_shape_offset = ordered_prims.len();
            for i in start..end {
                let prim_num = info_array[i].primitive_number;
                ordered_prims.push(prim_num);
            }
            let node = BVHBuildNode::init_leaf(first_shape_offset, n_shapes, bounds);
            return Box::new(node);
        } else {
            // Compute bound of primitive centroids
            let mut centroid_bounds = Bounds3::new(Vec3::ZEROS, Vec3::ZEROS);
            for i in start..end {
                centroid_bounds = centroid_bounds.union_vec(&info_array[i].centroid);
            }
            let ax = centroid_bounds.maximum_extent();

            let mid = (start + end) / 2;
            if centroid_bounds.pmax.get_axis(ax) == centroid_bounds.pmin.get_axis(ax) {
                // Create leaf BVH node
                let first_shape_offset = ordered_prims.len();
                for i in start..end {
                    let prim_num = info_array[i].primitive_number;
                    ordered_prims.push(prim_num);
                }
                let node = BVHBuildNode::init_leaf(first_shape_offset, n_shapes, bounds);
                return Box::new(node);
            } else {
                // Partition primitives based on split method
                
                let split_method = BVHSplitMethod::Equal;
                match split_method {
                    BVHSplitMethod::Equal => {
                        //nth element. Takes start, middle and ending pointer as well as
                        //comparison.
                        //Orders array so that element at middle pointer is the on
						println!("equal_method: start {} \t mid {} \t end {} info_array.len() {}", start, mid, end, info_array.len());
                        order_stat::kth_by(&mut info_array[start..end], mid-start, 
                            |a, b| a.centroid.get_axis(ax).partial_cmp(&b.centroid.get_axis(ax)).unwrap()
                        );
                    },
                    _ => {panic!("no implementation for split method.");}
                }

                let node = BVHBuildNode::init_interior(ax, 
                                   self.recursive_build(info_array, start, mid,
                                                   total_nodes, ordered_prims),
                                   self.recursive_build(info_array, mid, end,
                                                    total_nodes, ordered_prims)
                                   );
                return Box::new(node);
            }
        }
        //node
    }
}
