use derive_more::{Add, AddAssign, Sub, SubAssign};
use serde::{Deserialize, Serialize};

#[derive(Default, Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Add, AddAssign, Sub, SubAssign, Serialize, Deserialize)]
pub struct Id(pub u32);
impl Id {
    pub fn new(p0: u32) -> Self {
        Id(p0)
    }
}
