use std::fmt;

// use the "fixed" crate to create a fixed point number type
use fixed::{types::I24F8, traits::Fixed};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2I24F8{
    pub x : I24F8,
    pub y : I24F8,
}

#[derive(Clone, Copy, PartialEq)]
pub struct FixedFloat(I24F8);

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






#[cfg(test)]
pub mod data_types_tests{
    use std::collections::{HashSet, HashMap, LinkedList};

    use fixed::{types::I24F8};

    use super::FixedFloat;    

    pub struct MyStruct{
        pub a : i32,
        pub b : i32,
        pub child: Option<Box<MyStruct>>,
        pub hash_map : HashMap<i32, MyStruct>,
    }

    // override debug view of Baran
    impl std::fmt::Debug for MyStruct{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("MyStruct")
            // just print a static string
             .field("hello", &"world")
             .finish()
        }
    }
    // Q: its still the default view, its not showing me my "hello" fields. Why?
    // A: because you need to implement the Debug trait for the type of the field.


    #[test]
    pub fn asdasdas(){
        let a = FixedFloat::new(1.0);
        let b = FixedFloat::new(2.0);

        // define a vec of boxed floats
        let mut vec = Vec::new();
        // create barans
        let mut baran = MyStruct{a : 1, b : 2, child : None, hash_map : HashMap::new()};
        let mut baran2 = MyStruct{a : 3, b : 4, child : None, hash_map : HashMap::new()};
        let mut baran3 = MyStruct{a : 3, b : 4, child : None, hash_map : HashMap::new()};
        baran2.child = Some(Box::new(baran));
        baran2.hash_map.insert(1,baran3);
        
        // push them into the vec
        vec.push(Box::new(baran2));


        let mut hash_map_test = HashMap::new();
        let baran_0_0 = MyStruct{a : 1, b : 2, child : None, hash_map : HashMap::new()};
        let baran_0_1 = MyStruct{a : 3, b : 4, child : None, hash_map : HashMap::new()};
        hash_map_test.insert(1, baran_0_0);
        hash_map_test.insert(2, baran_0_1);
        
        // print baran_0_0
        let baran_0_2 = MyStruct{a : 1, b : 2, child : None, hash_map : HashMap::new()};
        println!("{:?}", baran_0_2);


        let c = a + b;
        
        let d = 1;

        // create a vec of numbers, push some numbers in it, turn it into an array
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        //let array = vec.into_boxed_slice();
        // not boxed slice, just array, here in the stack
        
        // create a vec of numbers, push some numbers in it, turn it into an array
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        // collect into a non boxed array
        let array : [i32; 3] = vec.into_iter().collect::<Vec<i32>>().try_into().unwrap();
        
        
        // create a linked list
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        



    }
}


//impl std::ops::Sub for Vector2I24F8{
//    type Output = Vector2I24F8;
//    fn sub(self, other : Vector2I24F8) -> Vector2I24F8{
//        Vector2I24F8{x : self.x - other.x, y : self.y - other.y}
//    }
//}
//
//impl std::ops::Add for Vector2I24F8{
//    type Output = Vector2I24F8;
//    fn add(self, other : Vector2I24F8) -> Vector2I24F8{
//        Vector2I24F8{x : self.x + other.x, y : self.y + other.y}
//    }
//}
//
//// support multiplication by a scalar
//impl std::ops::Mul<I24F8> for Vector2I24F8{
//    type Output = Vector2I24F8;
//    fn mul(self, other : I24F8) -> Vector2I24F8{
//        Vector2I24F8{x : self.x * other, y : self.y * other}
//    }
//}
//
//// support multiplication by a scalar
//impl std::ops::Mul<f32> for Vector2I24F8{
//    type Output = Vector2I24F8;
//    fn mul(self, other : f32) -> Vector2I24F8{
//        Vector2I24F8{x : self.x * other, y : self.y * other}
//    }
//}
//
//// support multiplication by a i32
//impl std::ops::Mul<i32> for Vector2I24F8{
//    type Output = Vector2I24F8;
//    fn mul(self, other : i32) -> Vector2I24F8{
//        Vector2I24F8{x : self.x * other, y : self.y * other}
//    }
//}

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

