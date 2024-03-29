// takes a seed and returns a cross platform deterministic random number without any dependencies
use crate::game_core::math::*;

pub struct Random {
    state: i64,
}

impl Random {
    pub fn seed_i64(seed: i64) -> Self {
        Self { state: seed }
    }
    pub fn seed_fixed_point(seed: FixedPoint) -> Self {
        Self {
            state: seed.to_bits(),
        }
    }

    pub fn next_i64(&mut self) -> i64 {
        self.state = random(self.state);
        self.state
    }

    pub fn next_i64_range(&mut self, min: i64, max: i64) -> i64 {
        let range = max - min;
        let random = self.next_i64();
        min + (random % range)
    }

    pub fn next_fixed_point(&mut self) -> FixedPoint {
        FixedPoint::from_bits(self.next_i64())
    }

    pub fn next_fixed_point_range(&mut self, min: FixedPoint, max: FixedPoint) -> FixedPoint {
        let range = max - min;
        let random = self.next_fixed_point();
        min + (random % range)
    }

    pub fn next_fixed_point_v2(&mut self) -> FixedPointV2 {
        FixedPointV2::new(self.next_fixed_point(), self.next_fixed_point())
    }

    pub fn next_fixed_point_on_unit_circle(&mut self) -> FixedPointV2 {
        let angle = self.next_fixed_point_range(FixedPoint::new(0.0), FixedPoint::new(2.0) * FixedPoint::pi());
        FixedPointV2::new(angle.cos(), angle.sin())
    }


}

fn random(seed: i64) -> i64 {
    let mut x = seed;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}