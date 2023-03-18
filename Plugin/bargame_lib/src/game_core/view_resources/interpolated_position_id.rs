use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Add, AddAssign, Sub, SubAssign)]
pub struct InterpolatedPositionId(u32);