use crate::core::*;

pub type Bounds3f = Bounds3<Real>;

pub struct Bounds3<T> {
    pmin: Vector3<T>,
    pmax: Vector3<T>,
}
impl Bounds3<T> {
    fn diagonal(&self) -> Vector3<T> {
        self.pmax - self.pmin
    }
}
