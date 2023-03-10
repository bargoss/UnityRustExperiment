use nalgebra::Vector2;
use nalgebra::Vector3;
use simba::scalar::{FixedI64};

type BaseType = FixedI64<fixed::types::extra::U24>;


pub struct FixedPoint(pub BaseType);

impl FixedPoint {
    pub(crate) fn new(p0: f64) -> Self {
        FixedPoint(FixedI64::<fixed::types::extra::U24>::from_num(p0))
    }
}

// impl deref for FixedPoint
impl std::ops::Deref for FixedPoint {
    type Target = BaseType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}



pub struct FixedPointV2(pub Vector2<BaseType>);

impl FixedPointV2 {
    pub(crate) fn new(p0: f64, p1: f64) -> Self {
        FixedPointV2(Vector2::new(BaseType::from_num(p0), BaseType::from_num(p1)))
    }
}

pub struct FixedPointV3(pub Vector3<BaseType>);


// make that an extension trait
pub trait FixedPointExt {
    fn floor_to_i32(&self) -> i32;
}
impl FixedPointExt for BaseType {
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
    fn test_deref() {
        let num0 = FixedPoint::new(0.5);
        let num1 = FixedPoint::new(1.5);
        
        let num2 = FixedPoint(*num0 + *num1);
    }

    #[test]
    fn test_0() {
        let num0 = FixedPoint::new(0.5);
        let num1 = FixedPoint::new(1.5);
        let vec0 = FixedPointV2::new(0.5, 1.5);
        let normalized_vec0 = vec0.0.normalize();
        let adsa = normalized_vec0 + normalized_vec0;
        println!("normalized_vec0: {:?}", normalized_vec0);
        println!("adsa: {:?}", adsa);

    }
}

