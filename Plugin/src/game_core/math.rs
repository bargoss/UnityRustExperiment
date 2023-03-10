use nalgebra::Vector2;
use nalgebra::Vector3;
use simba::scalar::{FixedI64};
use derive_more::{Deref, DerefMut, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

type BaseType = FixedI64<fixed::types::extra::U24>;

#[derive(Debug, Clone, Copy, Deref, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign)]
pub struct FixedPoint(pub BaseType);

impl FixedPoint {
    pub(crate) fn from_num(p0: f64) -> Self {
        FixedPoint(FixedI64::<fixed::types::extra::U24>::from_num(p0))
    }
}

#[derive(Debug, Clone, Copy, Deref, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign)]
pub struct FixedPointV2(Vector2<BaseType>);

impl FixedPointV2 {
    pub fn from_num(x: f64, y: f64) -> Self {
        FixedPointV2(Vector2::new(FixedI64::<fixed::types::extra::U24>::from_num(x), FixedI64::<fixed::types::extra::U24>::from_num(y)))
    }
}

//implement multipli

#[derive(Debug, Clone, Copy, Deref, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign)]
pub struct FixedPointV3(Vector3<BaseType>);

impl FixedPointV3 {
    pub fn from_num(x: f64, y: f64, z: f64) -> Self {
        FixedPointV3(Vector3::new(FixedI64::<fixed::types::extra::U24>::from_num(x), FixedI64::<fixed::types::extra::U24>::from_num(y), FixedI64::<fixed::types::extra::U24>::from_num(z)))
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



macro_rules! create_derive(
    ($feature:literal, $mod_:ident, $trait_:ident, $fn_name: ident $(,$attribute:ident)* $(,)?) => {
        #[cfg(feature = $feature)]
        #[proc_macro_derive($trait_, attributes($($attribute),*))]
        #[doc(hidden)]
        pub fn $fn_name(input: TokenStream) -> TokenStream {
            let ast = syn::parse(input).unwrap();
            Output::process($mod_::expand(&ast, stringify!($trait_)))
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref() {
        let num0 = FixedPoint::from_num(0.5);
        let num1 = FixedPoint::from_num(1.5);
        let vec0 = FixedPointV2::from_num(0.5, 1.5);
        let vec1 = FixedPointV2::from_num(1.5, 2.5);
        // sum of two vectors
        let vec2 = vec0 + vec1;

        println!("num0: {:?}", num0);
        println!("num1: {:?}", num1);
    }


    #[test]
    fn test_0() {
        let num0 = FixedPoint::from_num(0.5);
        let num1 = FixedPoint::from_num(1.5);
        let vec0 = FixedPointV2::from_num(0.5, 1.5);
        let normalized_vec0 = vec0.0.normalize();
        let adsa = normalized_vec0 + normalized_vec0;
        println!("normalized_vec0: {:?}", normalized_vec0);
        println!("adsa: {:?}", adsa);

    }
}


use nphysics2d::simba::scalar::RealField;


// implement RealField for FixedPoint

//impl RealField for FixedPoint
