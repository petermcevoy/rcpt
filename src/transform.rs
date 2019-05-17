use crate::{
    Vec3,
    Quaternion
};
use nalgebra as na;

pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quaternion
}

impl Transform {
    fn new(translation: Vec3, rotation: Quaternion) -> Transform {
        Transform {
            translation,
            rotation
        }
    }

    fn apply(self, v: Vec3) -> Vec3 {
        self.rotation.transform_vec(v) + self.translation
    }
    fn apply_reverse(self, v: Vec3) -> Vec3 {
        self.rotation.inv().transform_vec(v - self.translation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_apply() {
        let t = Transform::new(
            Vec3(1.0, 0.0, 0.0),
            Quaternion::from_eulerangles(Vec3(0.0, PI/2.0, 0.0))
        );
        assert_eq!(t.apply(Vec3(2.0, 0.0, 0.0)), Vec3(1.0, 0.0, -2.0));
    }
}
