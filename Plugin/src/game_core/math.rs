use nalgebra::Vector2;
use nalgebra::Vector3;
use simba::scalar::FixedI40F24;

// type FixedI40F24
pub type FixedPoint = FixedI40F24;
pub type FixedPointV2 = Vector2<FixedPoint>;
pub type FixedPointV3 = Vector3<FixedPoint>;

//pub fn floor_to_i32(value: FixedPoint) -> i32 {
//    let integer_bits = value.to_bits() >> 24;
//    let integer_bits = integer_bits as i32;
//    integer_bits;
//}

// make that an extension trait
pub trait FixedPointExt {
    fn floor_to_i32(&self) -> i32;
}
impl FixedPointExt for FixedPoint {
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
        let vec0 = FixedPointV2::new(num0, num1);
        let normalized_vec0 = vec0.normalize();
        let adsa = normalized_vec0 + normalized_vec0;
        println!("normalized_vec0: {:?}", normalized_vec0);
        println!("adsa: {:?}", adsa);

    }
}


use nphysics2d::simba::scalar::RealField;


// implement RealField for FixedPoint

//impl RealField for FixedPoint
