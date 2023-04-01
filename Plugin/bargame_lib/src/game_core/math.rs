use std::hash::{Hash, Hasher};
use std::ops::{Div, Mul};
use nalgebra::Vector2;
use nalgebra::Vector3;
use derive_more::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use simba::scalar::ComplexField;
use serde::{Serialize, Deserialize, Deserializer};
use serde::ser::SerializeTuple;

type BaseType = simba::scalar::FixedI40F24;

#[derive(Debug, Clone, Copy, Add, Sub, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg)]
pub struct FP(BaseType);

impl Hash for FP {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}


// implement Rem between FixedPoint and FixedPoint
impl std::ops::Rem for FP {
    type Output = FP;
    fn rem(self, rhs: FP) -> Self::Output {
        FP(self.0 % rhs.0)
    }
}


// impl Multiplication with u32
impl Mul<u32> for FP {
    type Output = FP;
    fn mul(self, rhs: u32) -> Self::Output {
        FP(self.0 * FP::new(rhs as f64).0)
    }
}

impl std::fmt::Display for FP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "{}", self.0.0.to_num::<f64>())
        write!(f, "{}", self.to_f64())
    }
}

impl From<i32> for FP {
    fn from(i: i32) -> Self {
        FP(BaseType::from_num(i as f64))
    }
}

impl From<FP> for f32 {
    fn from(fixed_point: FP) -> Self {
        fixed_point.to_f32()
    }
}

// impl serialize and deserialize
impl Serialize for FP {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.0.to_bits())
    }
}
impl<'de> Deserialize<'de> for FP {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let i = i64::deserialize(deserializer)?;
        Ok(FP(BaseType::from_bits(i)))
    }
}

// impl PartialOrd, PartialEq
impl PartialOrd for FP {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl PartialEq for FP {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

// impl Div with FixedPoint and FixedPoint
impl Div<FP> for FP {
    type Output = FP;
    fn div(self, rhs: FP) -> Self::Output {
        FP(self.0 / rhs.0)
    }
}

// impl Mul with FixedPoint and FixedPoint
impl Mul<FP> for FP {
    type Output = FP;
    fn mul(self, rhs: FP) -> Self::Output {
        FP(self.0 * rhs.0)
    }
}




impl FP {
    pub fn new(p0: f64) -> Self {
        FP(BaseType::from_num(p0))
    }

    pub fn from_num(p0: f64) -> Self {
        FP(BaseType::from_num(p0))
    }

    // one
    pub fn one() -> Self {
        FP(BaseType::from_num(1.0))
    }

    // to bits
    pub fn to_bits(&self) -> i64 {
        self.0.to_bits()
    }

    // lerp
    pub fn lerp(a: FP, b: FP, t: FP) -> FP {
        // dont use BaseType::lerp
        FP(a.0 + (b.0 - a.0) * t.0)
    }

    // inverse lerp
    pub fn inverse_lerp(a: FP, b: FP, t: FP) -> FP {
        // dont use BaseType::inverse_lerp
        FP((t.0 - a.0) / (b.0 - a.0))
    }

    // remap from0, from1, to0, to1
    pub fn remap(from0: FP, from1: FP, to0: FP, to1: FP, value: FP) -> FP {
        let t = FP::inverse_lerp(from0, from1, value);
        FP::lerp(to0, to1, t)
    }

    // clamp01
    pub fn clamp01(&self) -> FP {
        FP::clamp(*self, FP::zero(), FP::one())
    }


    //impl .cos(), .sin()
    pub fn cos(&self) -> Self {
        FP(self.0.cos())
    }
    pub fn sin(&self) -> Self {
        FP(self.0.sin())
    }

    // impl PI
    pub fn pi() -> Self {
        FP(BaseType::from_num(std::f64::consts::PI))
    }

    // from bits
    pub fn from_bits(bits: i64) -> Self {
        FP(BaseType::from_bits(bits))
    }

    pub fn floor_to_i32(&self) -> i32 {
        self.0.floor().0.to_num() // sus
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
        FP(BaseType::from_num(0.0))
    }

    pub fn clamp(self, min: FP, max: FP) -> FP {
        FP(self.0.clamp(min.0, max.0))
    }
}

//impl Default
impl Default for FP {
    fn default() -> Self {
        FP(BaseType::from_num(0.0))
    }
}


#[derive(Debug, Clone, Copy, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg)]
pub struct FP2(Vector2<BaseType>);

impl Hash for FP2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

// implement multiplication via FixedPoint
impl Mul<FP> for FP2 {
    type Output = FP2;
    fn mul(self, rhs: FP) -> Self::Output {
        FP2(self.0 * rhs.0)
    }
}

// impl display
impl std::fmt::Display for FP2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x().to_f64(), self.y().to_f64())
    }
}

// make FixedPointV2 convertable to FixedPointV3
impl From<FP2> for FP3 {
    fn from(fixed_point_v2: FP2) -> Self {
        FP3(Vector3::new(fixed_point_v2.0.x, fixed_point_v2.0.y, BaseType::from_num(0.0)))
    }
}



// impl Div for FixedPointV2 / FixedPoint
impl Div<FP> for FP2 {
    type Output = FP2;
    fn div(self, rhs: FP) -> Self::Output {
        FP2(self.0 / rhs.0)
    }
}


impl Mul<FP2> for FP {
    type Output = FP2;
    fn mul(self, rhs: FP2) -> Self::Output {
        let x = self * rhs.x();
        let y = self * rhs.y();
        FP2(Vector2::new(x.0, y.0))
    }
}

impl Serialize for FP2 {
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
impl<'de> Deserialize<'de> for FP2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let (x_bits, y_bits): (i64, i64) = Deserialize::deserialize(deserializer)?;
        let x = BaseType::from_bits(x_bits);
        let y = BaseType::from_bits(y_bits);
        return Ok(FP2 {
            0: Vector2::new(x, y)
        });


        //Ok(FixedPointV2(FixedPoint { x, y }))
    }
}

impl Serialize for FP3 {
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

impl<'de> Deserialize<'de> for FP3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let (x_bits, y_bits, z_bits): (i64, i64, i64) = Deserialize::deserialize(deserializer)?;
        let x = BaseType::from_bits(x_bits);
        let y = BaseType::from_bits(y_bits);
        let z = BaseType::from_bits(z_bits);
        return Ok(FP3 {
            0: Vector3::new(x, y, z)
        });
    }
}


impl FP2 {
    pub fn magnitude(&self) -> FP {
        FP(self.0.magnitude())
    }
    pub fn magnitude_squared(&self) -> FP {
        FP(self.0.magnitude_squared())
    }
    pub fn normalize(&self) -> FP2 {
        FP2(self.0.normalize())
    }
    pub fn safe_normalize(&self) -> FP2 {
        let sqr_magnitude = self.magnitude_squared();
        if sqr_magnitude.0 > BaseType::from_num(0.0) {
            FP2(self.0 / sqr_magnitude.0.sqrt())
        } else {
            FP2::zero()
        }
    }

    // perpendicular
    pub fn perp(&self) -> FP2 {
        FP2(Vector2::new(-self.0.y, self.0.x))
    }
}

//impl Default
impl Default for FP2 {
    fn default() -> Self {
        FP2(Vector2::new(BaseType::from_num(0.0), BaseType::from_num(0.0)))
    }
}

impl FP2 {
    pub fn new(x: FP, y: FP) -> Self {
        FP2(Vector2::new(x.0, y.0))
    }
    pub fn from_num(p0: f64, p1: f64) -> Self {
        FP2(Vector2::new(BaseType::from_num(p0), BaseType::from_num(p1)))
    }

    // getter and setter for x and y
    pub fn x(&self) -> FP {
        FP(self.0.x)
    }
    pub fn y(&self) -> FP {
        FP(self.0.y)
    }
    pub fn set_x(&mut self, x: FP) {
        self.0.x = x.0;
    }
    pub fn set_y(&mut self, y: FP) {
        self.0.y = y.0;
    }

    pub fn zero() -> Self {
        FP2(Vector2::new(BaseType::from_num(0.0), BaseType::from_num(0.0)))
    }
}



#[derive(Debug, Clone, Copy, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg, PartialEq)]
pub struct FP3(Vector3<BaseType>);

impl Hash for FP3 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

// impl display
impl std::fmt::Display for FP3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x().to_f64(), self.y().to_f64(), self.z().to_f64())
    }
}

impl FP3 {
    pub fn new(x: FP, y: FP, z: FP) -> Self {
        FP3(Vector3::new(x.0, y.0, z.0))
    }
    // mag
    pub fn magnitude(&self) -> FP {
        FP(self.0.magnitude())
    }
    pub fn magnitude_squared(&self) -> FP {
        FP(self.0.magnitude_squared())
    }
    pub fn from_num(p0: f64, p1: f64, p2: f64) -> Self {
        FP3(Vector3::new(BaseType::from_num(p0), BaseType::from_num(p1), BaseType::from_num(p2)))
    }
    pub fn normalize(&self) -> FP3 {
        FP3(self.0.normalize())
    }
    pub fn safe_normalize(&self) -> FP3 {
        let sqr_magnitude = self.magnitude_squared();
        if sqr_magnitude.0 > BaseType::from_num(0.0) {
            FP3(self.0 / sqr_magnitude.0.sqrt())
        } else {
            FP3::zero()
        }
    }

    // getter and setter for x and y
    pub fn x(&self) -> FP {
        FP(self.0.x)
    }
    pub fn y(&self) -> FP {
        FP(self.0.y)
    }
    pub fn z(&self) -> FP {
        FP(self.0.z)
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
        FP3(Vector3::new(BaseType::from_num(0.0), BaseType::from_num(0.0), BaseType::from_num(0.0)))
    }
}

//impl Default
impl Default for FP3 {
    fn default() -> Self {
        FP3(Vector3::new(BaseType::from_num(0.0), BaseType::from_num(0.0), BaseType::from_num(0.0)))
    }
}

// make that an extension trait
pub trait FPExt {
    fn floor_to_i32(&self) -> i32;
}
impl FPExt for BaseType {
    fn floor_to_i32(&self) -> i32 {
        let integer_bits = self.to_bits() >> 24;
        let integer_bits = integer_bits as i32;
        integer_bits
    }
}

impl FPExt for FP {
    fn floor_to_i32(&self) -> i32 {
        self.0.floor_to_i32()
    }
}

impl Mul<FP> for FP3 {
    type Output = FP3;
    fn mul(self, rhs: FP) -> Self::Output {
        FP3(self.0 * rhs.0)
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref() {
        let num0 = FP::new(0.5);
        let num1 = FP::new(1.5);
        
        let _num2 = num0 + num1;
    }

    #[test]
    fn test_0() {
        let _num0 = FP::new(0.5);
        let _num1 = FP::new(1.5);
        let vec0 = FP2::from_num(0.5, 1.5);
        let normalized_vec0 = vec0.0.normalize();
        let adsa = normalized_vec0 + normalized_vec0;
        println!("normalized_vec0: {:?}", normalized_vec0);
        println!("adsa: {:?}", adsa);

    }
}

