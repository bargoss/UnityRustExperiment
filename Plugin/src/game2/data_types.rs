use std::fmt;

// use the "fixed" crate to create a fixed point number type
use fixed::{types::I24F8, traits::Fixed};
use bevy_math::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2FixedFloat{
    pub x : FixedFloat,
    pub y : FixedFloat,
}

// implement math stuff for Vector2I24F8
impl Vector2FixedFloat{
    pub fn sqr_magnitude(&self) -> FixedFloat{
        self.x * self.x + self.y * self.y
    }
    
    pub fn magnitude(&self) -> FixedFloat{
        let sqr_mag = self.sqr_magnitude();
        sqr_mag.sqrt()
    }

    // normalize
    pub fn normalize(&self) -> Vector2FixedFloat{
        let length = self.magnitude();
        Vector2FixedFloat{
            x : self.x / length,
            y : self.y / length,
        }
    }
}

impl std::ops::Add for Vector2FixedFloat{
    type Output = Vector2FixedFloat;
    fn add(self, other : Vector2FixedFloat) -> Vector2FixedFloat{
        Vector2FixedFloat{
            x : self.x + other.x,
            y : self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vector2FixedFloat{
    type Output = Vector2FixedFloat;
    fn sub(self, other : Vector2FixedFloat) -> Vector2FixedFloat{
        Vector2FixedFloat{
            x : self.x - other.x,
            y : self.y - other.y,
        }
    }
}

// multiply with scalar
impl std::ops::Mul<FixedFloat> for Vector2FixedFloat{
    type Output = Vector2FixedFloat;
    fn mul(self, other : FixedFloat) -> Vector2FixedFloat{
        Vector2FixedFloat{
            x : self.x * other,
            y : self.y * other,
        }
    }
}

// divide with scalar
impl std::ops::Div<FixedFloat> for Vector2FixedFloat{
    type Output = Vector2FixedFloat;
    fn div(self, other : FixedFloat) -> Vector2FixedFloat{
        Vector2FixedFloat{
            x : self.x / other,
            y : self.y / other,
        }
    }
}











#[derive(Clone, Copy, PartialEq)]
pub struct FixedFloat(I24F8);

// implicit conversion from f32 to FixedFloat
impl From<f32> for FixedFloat{
    fn from(value : f32) -> FixedFloat{
        FixedFloat(I24F8::from_num(value))
    }
}

// implicit conversion from FixedFloat to f32
impl From<FixedFloat> for f32{
    fn from(value : FixedFloat) -> f32{
        value.0.to_num::<f32>()
    }
}

impl fmt::Debug for FixedFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
         .field(&self.0.to_num::<f32>())
         .finish()
    }
}

impl FixedFloat{
    pub fn new(value : f32) -> FixedFloat{
        FixedFloat(I24F8::from_num(value))
    }

    // implement sqrt
    pub fn sqrt(&self) -> FixedFloat{
        let as_float = self.0.to_num::<f32>();
        let as_float_sqrt = as_float.sqrt();
        FixedFloat(I24F8::from_num(as_float_sqrt))
    }

    // abs
    pub fn abs(&self) -> FixedFloat{
        let as_float = self.0.to_num::<f32>();
        let as_float_abs = as_float.abs();
        FixedFloat(I24F8::from_num(as_float_abs))
    }
}

impl std::ops::Add for FixedFloat{
    type Output = FixedFloat;
    fn add(self, other : FixedFloat) -> FixedFloat{
        FixedFloat(self.0 + other.0)
    }
}

impl std::ops::Sub for FixedFloat{
    type Output = FixedFloat;
    fn sub(self, other : FixedFloat) -> FixedFloat{
        FixedFloat(self.0 - other.0)
    }
}

impl std::ops::Mul for FixedFloat{
    type Output = FixedFloat;
    fn mul(self, other : FixedFloat) -> FixedFloat{
        FixedFloat(self.0 * other.0)
    }
}

impl std::ops::Div for FixedFloat{
    type Output = FixedFloat;
    fn div(self, other : FixedFloat) -> FixedFloat{
        FixedFloat(self.0 / other.0)
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2Int{
    pub x : i32,
    pub y : i32,
}

impl std::ops::Sub for Vector2Int{
    type Output = Vector2Int;
    fn sub(self, other : Vector2Int) -> Vector2Int{
        Vector2Int{x : self.x - other.x, y : self.y - other.y}
    }
}
impl std::ops::Add for Vector2Int{
    type Output = Vector2Int;
    fn add(self, other : Vector2Int) -> Vector2Int{
        Vector2Int{x : self.x + other.x, y : self.y + other.y}
    }
}
// support multiplication by a scalar
impl std::ops::Mul<i32> for Vector2Int{
    type Output = Vector2Int;
    fn mul(self, other : i32) -> Vector2Int{
        Vector2Int{x : self.x * other, y : self.y * other}
    }
}




#[cfg(test)]
pub mod data_types_tests{
    use std::collections::{HashSet, HashMap, LinkedList};

    use fixed::{types::I24F8};

    use super::FixedFloat;    



    #[test]
    pub fn tests(){
        let a = FixedFloat::new(1.0);
        let b = FixedFloat::new(2.0);
        let c = FixedFloat::new(3.0);
        let d = FixedFloat::new(4.0);

        let e = a + b;
        let f = c - d;

        assert_eq!(e, FixedFloat::new(3.0));
        assert_eq!(f, FixedFloat::new(-1.0));
    }

}
