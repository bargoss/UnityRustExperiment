use nalgebra::Vector2;
use nalgebra::Vector3;

type BaseType = simba::scalar::FixedI40F24;

#[derive(Debug, Clone, Copy)]
pub struct FixedPoint(pub BaseType);

impl FixedPoint {
    pub(crate) fn new(p0: f64) -> Self {
        FixedPoint(BaseType::from_num(p0))
    }
}

//impl Default
impl Default for FixedPoint {
    fn default() -> Self {
        FixedPoint(BaseType::from_num(0.0))
    }
}

// impl deref for FixedPoint
impl std::ops::Deref for FixedPoint {
    type Target = BaseType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}



#[derive(Debug, Clone, Copy)]
pub struct FixedPointV2(pub Vector2<BaseType>);

//impl Default
impl Default for FixedPointV2 {
    fn default() -> Self {
        FixedPointV2(Vector2::new(BaseType::from_num(0.0), BaseType::from_num(0.0)))
    }
}

impl FixedPointV2 {
    pub(crate) fn new(p0: f64, p1: f64) -> Self {
        FixedPointV2(Vector2::new(BaseType::from_num(p0), BaseType::from_num(p1)))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FixedPointV3(pub Vector3<BaseType>);

//impl Default
impl Default for FixedPointV3 {
    fn default() -> Self {
        FixedPointV3(Vector3::new(BaseType::from_num(0.0), BaseType::from_num(0.0), BaseType::from_num(0.0)))
    }
}


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

