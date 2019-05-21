use crate::core::*;

#[derive(Copy, Clone, Debug)]
pub struct Bounds3 {
    pub pmin: Vec3,
    pub pmax: Vec3,
}

impl Bounds3 {
    pub fn new(pmin: Vec3, pmax: Vec3) -> Bounds3 {
        Bounds3{pmin, pmax}
    }
    pub fn diagonal(&self) -> Vec3 {
        self.pmax - self.pmin
    }
    pub fn surface_area(&self) -> Real {
        let d = self.diagonal();
        2.0 * (d.x() * d.y() + d.x() * d.z() + d.y() * d.z())
    }
    pub fn maximum_extent(&self) -> Vec3Axis {
        let d = self.diagonal();
        if d.x() > d.y() && d.x() > d.z() {
            Vec3Axis::X 
        } else if d.y() > d.z() {
            Vec3Axis::Y
        } else {
            Vec3Axis::Z
        }
    }

    pub fn union(a: &Bounds3, b: &Bounds3) -> Bounds3 {
        let mut new_bound = *a;
        let axes: [Vec3Axis; 3] = [Vec3Axis::X, Vec3Axis::Y, Vec3Axis::Z];

        for ax in axes.iter() {
            if b.pmin.get_axis(*ax) < a.pmin.get_axis(*ax) {
                new_bound.pmin.set_axis(*ax, b.pmin.get_axis(*ax));
            }
            if b.pmax.get_axis(*ax) > a.pmax.get_axis(*ax) {
                new_bound.pmax.set_axis(*ax, b.pmax.get_axis(*ax));
            }
        }
        new_bound
    }
    
    pub fn union_vec(&self, b: &Vec3) -> Bounds3 {
        let mut new_bound = *self;
        let a = self;
        let axes: [Vec3Axis; 3] = [Vec3Axis::X, Vec3Axis::Y, Vec3Axis::Z];

        for ax in axes.iter() {
            if b.get_axis(*ax) < a.pmin.get_axis(*ax) {
                new_bound.pmin.set_axis(*ax, b.get_axis(*ax));
            }
            if b.get_axis(*ax) > a.pmax.get_axis(*ax) {
                new_bound.pmax.set_axis(*ax, b.get_axis(*ax));
            }
        }
        new_bound
    }
    
    fn gamma(n: u32) -> Real {
        let fac = (n as Real) * EPS;
        fac / (1.0 - fac)
    }

    //fn intersect_p(&self, ray: &Ray, inv_dir: &Vec3,
    //               dir_is_neg: &[bool; 3]) -> bool {
    //    let bounds = &self;
    //    let t_min = (bounds)

    //}
    pub fn intersect_p(&self, ray: &Ray, t_max: &Real) -> bool {
        let mut t0: Real = 0.0;
        let mut t1: Real = *t_max;

        for ax in Vec3Axis::iter() {
            let inv_ray_dir = 1.0 / ray.direction.get_axis(*ax);
            let mut t_near  = ( self.pmin.get_axis(*ax) - ray.origin.get_axis(*ax) ) * inv_ray_dir;
            let mut t_far   = ( self.pmax.get_axis(*ax) - ray.origin.get_axis(*ax) ) * inv_ray_dir;

            if t_near > t_far { std::mem::swap(&mut t_near, &mut t_far); }
            t_far *= 1.0 + 2.0 * Bounds3::gamma(3);

            t0 = if t_near  > t0 {t_near} else {t0};
            t1 = if t_far   < t1 {t_far} else {t1};
            if t0 > t1 {return false;}
        }
        return true;
    }
}
