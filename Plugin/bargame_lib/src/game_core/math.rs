use std::ops::{Div, Mul};
use nalgebra::Vector2;
use nalgebra::Vector3;
use derive_more::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use simba::scalar::ComplexField;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeTuple;

type BaseType = simba::scalar::FixedI40F24;

#[derive(Debug, Clone, Copy, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg)]
pub struct FixedPoint(BaseType);

impl std::fmt::Display for FixedPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "{}", self.0.0.to_num::<f64>())
        write!(f, "{}", self.to_f64())
    }
}

impl From<i32> for FixedPoint {
    fn from(i: i32) -> Self {
        FixedPoint(BaseType::from_num(i as f64))
    }
}

impl From<FixedPoint> for f32 {
    fn from(fixed_point: FixedPoint) -> Self {
        fixed_point.to_f32()
    }
}

// impl serialize and deserialize
impl Serialize for FixedPoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.0.to_bits())
    }
}
impl<'de> Deserialize<'de> for FixedPoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let i = i64::deserialize(deserializer)?;
        Ok(FixedPoint(BaseType::from_bits(i)))
    }
}

// impl PartialOrd, PartialEq
impl PartialOrd for FixedPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl PartialEq for FixedPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

// impl Div with FixedPoint and FixedPoint
impl Div<FixedPoint> for FixedPoint {
    type Output = FixedPoint;
    fn div(self, rhs: FixedPoint) -> Self::Output {
        FixedPoint(self.0 / rhs.0)
    }
}

// impl Mul with FixedPoint and FixedPoint
impl Mul<FixedPoint> for FixedPoint {
    type Output = FixedPoint;
    fn mul(self, rhs: FixedPoint) -> Self::Output {
        FixedPoint(self.0 * rhs.0)
    }
}


impl FixedPoint {
    fn floor_to_i32(&self) -> i32 {
        self.0.floor().0.to_num() // sus
    }
}

impl FixedPoint {
    pub fn new(p0: f64) -> Self {
        FixedPoint(BaseType::from_num(p0))
    }

    // one
    pub fn one() -> Self {
        FixedPoint(BaseType::from_num(1.0))
    }

    // to_f32
    pub fn to_f32(&self) -> f32 {
        // convert simba::scalar::FixedI40F24 to f32
        self.0.0.to_num()
    }
    // to f64
    pub fn to_f64(&self) -> f64 {
        self.0.0.to_num()
    }

    // zero
    pub fn zero() -> Self {
        FixedPoint(BaseType::from_num(0.0))
    }
}

//impl Default
impl Default for FixedPoint {
    fn default() -> Self {
        FixedPoint(BaseType::from_num(0.0))
    }
}


#[derive(Debug, Clone, Copy, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg)]
pub struct FixedPointV2(Vector2<BaseType>);

// impl display
impl std::fmt::Display for FixedPointV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x().to_f64(), self.y().to_f64())
    }
}

// make FixedPointV2 convertable to FixedPointV3
impl From<FixedPointV2> for FixedPointV3 {
    fn from(fixed_point_v2: FixedPointV2) -> Self {
        FixedPointV3(Vector3::new(fixed_point_v2.0.x, fixed_point_v2.0.y, BaseType::from_num(0.0)))
    }
}



// impl Div for FixedPointV2 / FixedPoint
impl Div<FixedPoint> for FixedPointV2 {
    type Output = FixedPointV2;
    fn div(self, rhs: FixedPoint) -> Self::Output {
        FixedPointV2(self.0 / rhs.0)
    }
}

// impl Mul for FixedPointV2 * FixedPoint
impl Mul<FixedPoint> for FixedPointV2 {
    type Output = FixedPointV2;
    fn mul(self, rhs: FixedPoint) -> Self::Output {
        FixedPointV2(self.0 * rhs.0)
    }
}

impl Serialize for FixedPointV2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        // Serialize the two i64 values into a tuple
        let mut tuple = serializer.serialize_tuple(2)?;
        tuple.serialize_element(&self.0.x.to_bits())?;
        tuple.serialize_element(&self.0.y.to_bits())?;
        tuple.end()
    }
}
impl<'de> Deserialize<'de> for FixedPointV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let (x_bits, y_bits): (i64, i64) = Deserialize::deserialize(deserializer)?;
        let x = BaseType::from_bits(x_bits);
        let y = BaseType::from_bits(y_bits);
        return Ok(FixedPointV2{
            0: Vector2::new(x, y)
        });


        //Ok(FixedPointV2(FixedPoint { x, y }))
    }
}

impl Serialize for FixedPointV3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        // Serialize the two i64 values into a tuple
        let mut tuple = serializer.serialize_tuple(3)?;
        tuple.serialize_element(&self.0.x.to_bits())?;
        tuple.serialize_element(&self.0.y.to_bits())?;
        tuple.serialize_element(&self.0.z.to_bits())?;
        tuple.end()
    }
}

impl<'de> Deserialize<'de> for FixedPointV3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let (x_bits, y_bits, z_bits): (i64, i64, i64) = Deserialize::deserialize(deserializer)?;
        let x = BaseType::from_bits(x_bits);
        let y = BaseType::from_bits(y_bits);
        let z = BaseType::from_bits(z_bits);
        return Ok(FixedPointV3{
            0: Vector3::new(x, y, z)
        });
    }
}


impl FixedPointV2 {
    pub fn magnitude(&self) -> FixedPoint {
        FixedPoint(self.0.magnitude())
    }
    pub fn magnitude_squared(&self) -> FixedPoint {
        FixedPoint(self.0.magnitude_squared())
    }
    pub fn normalize(&self) -> FixedPointV2 {
        FixedPointV2(self.0.normalize())
    }
    pub fn safe_normalize(&self) -> FixedPointV2 {
        let sqr_magnitude = self.magnitude_squared();
        if sqr_magnitude.0 > BaseType::from_num(0.0) {
            FixedPointV2(self.0 / sqr_magnitude.0.sqrt())
        } else {
            FixedPointV2::zero()
        }
    }
}

//impl Default
impl Default for FixedPointV2 {
    fn default() -> Self {
        FixedPointV2(Vector2::new(BaseType::from_num(0.0), BaseType::from_num(0.0)))
    }
}

impl FixedPointV2 {
    pub fn new(x: FixedPoint, y: FixedPoint) -> Self {
        FixedPointV2(Vector2::new(x.0, y.0))
    }
    pub fn from_num(p0: f64, p1: f64) -> Self {
        FixedPointV2(Vector2::new(BaseType::from_num(p0), BaseType::from_num(p1)))
    }

    // getter and setter for x and y
    pub fn x(&self) -> FixedPoint {
        FixedPoint(self.0.x)
    }
    pub fn y(&self) -> FixedPoint {
        FixedPoint(self.0.y)
    }
    pub fn set_x(&mut self, x: FixedPoint) {
        self.0.x = x.0;
    }
    pub fn set_y(&mut self, y: FixedPoint) {
        self.0.y = y.0;
    }

    pub fn zero() -> Self {
        FixedPointV2(Vector2::new(BaseType::from_num(0.0), BaseType::from_num(0.0)))
    }
}



#[derive(Debug, Clone, Copy, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg, PartialEq)]
pub struct FixedPointV3(Vector3<BaseType>);

// impl display
impl std::fmt::Display for FixedPointV3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x().to_f64(), self.y().to_f64(), self.z().to_f64())
    }
}

impl FixedPointV3 {
    pub fn new(x: FixedPoint, y: FixedPoint, z: FixedPoint) -> Self {
        FixedPointV3(Vector3::new(x.0, y.0, z.0))
    }
    // mag
    pub fn magnitude(&self) -> FixedPoint {
        FixedPoint(self.0.magnitude())
    }
    pub fn magnitude_squared(&self) -> FixedPoint {
        FixedPoint(self.0.magnitude_squared())
    }
    pub fn from_num(p0: f64, p1: f64, p2: f64) -> Self {
        FixedPointV3(Vector3::new(BaseType::from_num(p0), BaseType::from_num(p1), BaseType::from_num(p2)))
    }
    pub fn normalize(&self) -> FixedPointV3 {
        FixedPointV3(self.0.normalize())
    }
    pub fn safe_normalize(&self) -> FixedPointV3 {
        let sqr_magnitude = self.magnitude_squared();
        if sqr_magnitude.0 > BaseType::from_num(0.0) {
            FixedPointV3(self.0 / sqr_magnitude.0.sqrt())
        } else {
            FixedPointV3::zero()
        }
    }

    // getter and setter for x and y
    pub fn x(&self) -> FixedPoint {
        FixedPoint(self.0.x)
    }
    pub fn y(&self) -> FixedPoint {
        FixedPoint(self.0.y)
    }
    pub fn z(&self) -> FixedPoint {
        FixedPoint(self.0.z)
    }
    pub fn set_x(&mut self, x: f64) {
        self.0.x = BaseType::from_num(x);
    }
    pub fn set_y(&mut self, y: f64) {
        self.0.y = BaseType::from_num(y);
    }
    pub fn set_z(&mut self, z: f64) {
        self.0.z = BaseType::from_num(z);
    }

    pub fn zero() -> Self {
        FixedPointV3(Vector3::new(BaseType::from_num(0.0), BaseType::from_num(0.0), BaseType::from_num(0.0)))
    }
}

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

impl FixedPointExt for FixedPoint {
    fn floor_to_i32(&self) -> i32 {
        self.0.floor_to_i32()
    }
}

impl Mul<FixedPoint> for FixedPointV3 {
    type Output = FixedPointV3;
    fn mul(self, rhs: FixedPoint) -> Self::Output {
        FixedPointV3(self.0 * rhs.0)
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref() {
        let num0 = FixedPoint::new(0.5);
        let num1 = FixedPoint::new(1.5);
        
        let num2 = num0 + num1;
    }

    #[test]
    fn test_0() {
        let num0 = FixedPoint::new(0.5);
        let num1 = FixedPoint::new(1.5);
        let vec0 = FixedPointV2::from_num(0.5, 1.5);
        let normalized_vec0 = vec0.0.normalize();
        let adsa = normalized_vec0 + normalized_vec0;
        println!("normalized_vec0: {:?}", normalized_vec0);
        println!("adsa: {:?}", adsa);

    }
}

