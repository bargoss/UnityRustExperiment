use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Add, AddAssign, Sub, SubAssign)]
pub struct Index(u32);
impl Index {
    pub fn new(p0: u32) -> Self {
        Index(p0)
    }
}