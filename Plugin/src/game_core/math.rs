use nalgebra::Vector2;
use nalgebra::Vector3;
use simba::scalar::{FixedI64};

// type FixedI40F24
//pub type FixedPoint = FixedI64<fixed::types::extra::U24>;
//pub type FixedPointV2 = Vector2<FixedPoint>;
//pub type FixedPointV3 = Vector3<FixedPoint>;
pub struct FixedPoint(pub FixedI64<fixed::types::extra::U24>);

impl FixedPoint {
    pub(crate) fn from_num(p0: f64) -> Self {
        FixedPoint(FixedI64::<fixed::types::extra::U24>::from_num(p0))
    }
}

pub struct FixedPointV2(pub Vector2<FixedI64<fixed::types::extra::U24>>);

impl FixedPointV2 {
    pub(crate) fn new(p0: f64, p1: f64) -> Self {
        FixedPointV2(Vector2::new(FixedI64::<fixed::types::extra::U24>::from_num(p0), FixedI64::<fixed::types::extra::U24>::from_num(p1)))
    }
}

pub struct FixedPointV3(pub Vector3<FixedI64<fixed::types::extra::U24>>);


// make that an extension trait
pub trait FixedPointExt {
    fn floor_to_i32(&self) -> i32;
}
impl FixedPointExt for FixedI64<fixed::types::extra::U24> {
    fn floor_to_i32(&self) -> i32 {
        let integer_bits = self.to_bits() >> 24;
        let integer_bits = integer_bits as i32;
        integer_bits
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let num0 = FixedPoint::from_num(0.5);
        let num1 = FixedPoint::from_num(1.5);
        let vec0 = FixedPointV2::new(0.5, 1.5);
        let normalized_vec0 = vec0.0.normalize();
        let adsa = normalized_vec0 + normalized_vec0;
        println!("normalized_vec0: {:?}", normalized_vec0);
        println!("adsa: {:?}", adsa);

    }
}


use nphysics2d::simba::scalar::RealField;


// implement RealField for FixedPoint

//impl RealField for FixedPoint
