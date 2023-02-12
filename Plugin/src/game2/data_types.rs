use std::fmt;

// use the "fixed" crate to create a fixed point number type
use fixed::{types::I24F8, traits::Fixed};
use bevy_math::Vec2;

#[derive(Clone, Copy, PartialEq)]
pub struct Vec2FFloat{
    pub x : FFloat,
    pub y : FFloat,
}

// implement debug for Vec2FFloat
impl fmt::Debug for Vec2FFloat{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec2FFloat({:?}, {:?})", self.x, self.y)
    }
}

// implemenmt addassign for Vec2FFloat
impl std::ops::AddAssign for Vec2FFloat{
    fn add_assign(&mut self, other: Self){
        self.x += other.x;
        self.y += other.y;
    }
}

// implement math stuff for Vector2I24F8
impl Vec2FFloat{
    // constructor
    pub fn new(x : f32, y : f32) -> Vec2FFloat{
        Vec2FFloat{
            x : FFloat::new(x),
            y : FFloat::new(y),
        }
    }
    // return string
    pub fn display(&self) -> String{
        format!("({}, {})", f32::from(self.x), f32::from(self.y))
    }
    // zero vector
    pub fn zero() -> Vec2FFloat{
        Vec2FFloat{
            x : FFloat::new(0.0),
            y : FFloat::new(0.0),
        }
    }

    pub fn sqr_mag(&self) -> FFloat{
        self.x * self.x + self.y * self.y
    }
    
    pub fn mag(&self) -> FFloat{
        let sqr_mag = self.sqr_mag();
        sqr_mag.sqrt()
    }

    // normalize
    pub fn normalize(&self) -> Vec2FFloat{
        let length = self.mag();
        Vec2FFloat{
            x : self.x / length,
            y : self.y / length,
        }
    }
}

impl std::ops::Add for Vec2FFloat{
    type Output = Vec2FFloat;
    fn add(self, other : Vec2FFloat) -> Vec2FFloat{
        Vec2FFloat{
            x : self.x + other.x,
            y : self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vec2FFloat{
    type Output = Vec2FFloat;
    fn sub(self, other : Vec2FFloat) -> Vec2FFloat{
        Vec2FFloat{
            x : self.x - other.x,
            y : self.y - other.y,
        }
    }
}

// multiply with scalar
impl std::ops::Mul<FFloat> for Vec2FFloat{
    type Output = Vec2FFloat;
    fn mul(self, other : FFloat) -> Vec2FFloat{
        Vec2FFloat{
            x : self.x * other,
            y : self.y * other,
        }
    }
}

// divide with scalar
impl std::ops::Div<FFloat> for Vec2FFloat{
    type Output = Vec2FFloat;
    fn div(self, other : FFloat) -> Vec2FFloat{
        Vec2FFloat{
            x : self.x / other,
            y : self.y / other,
        }
    }
}











#[derive(Clone, Copy, PartialEq)]
pub struct FFloat(I24F8);

// implement PartialOrd
impl PartialOrd for FFloat{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

// implement addassign for FFloat
impl std::ops::AddAssign for FFloat{
    fn add_assign(&mut self, other: Self){
        self.0 += other.0;
    }
}

// implicit conversion from f32 to FixedFloat
impl From<f32> for FFloat{
    fn from(value : f32) -> FFloat{
        FFloat(I24F8::from_num(value))
    }
}

// implicit conversion from FixedFloat to f32
impl From<FFloat> for f32{
    fn from(value : FFloat) -> f32{
        value.0.to_num::<f32>()
    }
}

impl fmt::Debug for FFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
         .field(&self.0.to_num::<f32>())
         .finish()
    }
}

impl FFloat{
    pub fn new(value : f32) -> FFloat{
        FFloat(I24F8::from_num(value))
    }

    // implement sqrt
    pub fn sqrt(&self) -> FFloat{
        let as_float = self.0.to_num::<f32>();
        let as_float_sqrt = as_float.sqrt();
        FFloat(I24F8::from_num(as_float_sqrt))
    }

    // abs
    pub fn abs(&self) -> FFloat{
        let as_float = self.0.to_num::<f32>();
        let as_float_abs = as_float.abs();
        FFloat(I24F8::from_num(as_float_abs))
    }
}

impl std::ops::Add for FFloat{
    type Output = FFloat;
    fn add(self, other : FFloat) -> FFloat{
        FFloat(self.0 + other.0)
    }
}

impl std::ops::Sub for FFloat{
    type Output = FFloat;
    fn sub(self, other : FFloat) -> FFloat{
        FFloat(self.0 - other.0)
    }
}

impl std::ops::Mul for FFloat{
    type Output = FFloat;
    fn mul(self, other : FFloat) -> FFloat{
        FFloat(self.0 * other.0)
    }
}

impl std::ops::Div for FFloat{
    type Output = FFloat;
    fn div(self, other : FFloat) -> FFloat{
        FFloat(self.0 / other.0)
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
    use super::FFloat;
    use super::Vec2FFloat;



    #[test]
    pub fn tests(){
        let a = FFloat::new(1.0);
        let b = FFloat::new(2.0);
        let c = FFloat::new(3.0);
        let d = FFloat::new(4.0);

        let e = a + b;
        let f = c - d;

        assert_eq!(e, FFloat::new(3.0));
        assert_eq!(f, FFloat::new(-1.0));

        let v0 = Vec2FFloat{x : FFloat::new(4.0), y : FFloat::new(0.0)};
        let v1 = Vec2FFloat{x : FFloat::new(0.0), y : FFloat::new(3.0)};

        let v2 = v0 + v1;

        assert_eq!(v2, Vec2FFloat{x : FFloat::new(4.0), y : FFloat::new(3.0)});

        let sqr_mag = v2.sqr_mag();
        assert_eq!(sqr_mag, FFloat::new(25.0));

        let mag = v2.mag();
        assert_eq!(mag, FFloat::new(5.0));
    }

}
