// use the "fixed" crate to create a fixed point number type
use fixed::types::I24F8;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2I24F8{
    pub x : I24F8,
    pub y : I24F8,
}

impl std::ops::Sub for Vector2I24F8{
    type Output = Vector2I24F8;
    fn sub(self, other : Vector2I24F8) -> Vector2I24F8{
        Vector2I24F8{x : self.x - other.x, y : self.y - other.y}
    }
}

impl std::ops::Add for Vector2I24F8{
    type Output = Vector2I24F8;
    fn add(self, other : Vector2I24F8) -> Vector2I24F8{
        Vector2I24F8{x : self.x + other.x, y : self.y + other.y}
    }
}

// support multiplication by a scalar
impl std::ops::Mul<I24F8> for Vector2I24F8{
    type Output = Vector2I24F8;
    fn mul(self, other : I24F8) -> Vector2I24F8{
        Vector2I24F8{x : self.x * other, y : self.y * other}
    }
}

// support multiplication by a scalar
impl std::ops::Mul<f32> for Vector2I24F8{
    type Output = Vector2I24F8;
    fn mul(self, other : f32) -> Vector2I24F8{
        Vector2I24F8{x : self.x * other, y : self.y * other}
    }
}

// support multiplication by a i32
impl std::ops::Mul<i32> for Vector2I24F8{
    type Output = Vector2I24F8;
    fn mul(self, other : i32) -> Vector2I24F8{
        Vector2I24F8{x : self.x * other, y : self.y * other}
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

