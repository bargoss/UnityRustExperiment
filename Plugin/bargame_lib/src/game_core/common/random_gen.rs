use blake3;
use std::hash::{Hash, Hasher as StdHasher};
use crate::game_core::math::*;
use crate::game_core::common::Id;
use crate::game_core::components::NetId;

const ID_HASH_KEY: u32 = 455612;
const NET_ID_HASH_KEY: u32 = 943503;
const FIXED_POINT_HASH_KEY: u32 = 51321;
const FIXED_POINT_V2_HASH_KEY: u32 = 734534;
const FIXED_POINT_V3_HASH_KEY: u32 = 612312;

pub struct IdGenerator;

// Wrapper for blake3::Hasher that implements std::hash::Hasher
struct Blake3HasherWrapper(blake3::Hasher);

impl StdHasher for Blake3HasherWrapper {
    fn finish(&self) -> u64 {
        self.0.finalize().as_bytes()[0..8].iter().fold(0u64, |acc, &byte| {
            (acc << 8) | (byte as u64)
        })
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0.update(bytes);
    }
}

// IdGeneratorBuilder for the builder pattern
pub struct RandomGen {
    hasher: Blake3HasherWrapper,
}

fn u64_to_i64(hash: u64) -> i64 {
    (hash ^ (1u64 << 63)) as i64
}

impl RandomGen {
    // Initialize a new IdGeneratorBuilder
    pub fn start() -> Self {
        RandomGen {
            hasher: Blake3HasherWrapper(blake3::Hasher::new()),
        }
    }

    // Finish building and return the final hash
    pub fn finish_get_id(self) -> Id {
        let raw_hash = self.hasher.finish();

        Id::new(raw_hash as u32)
    }

    pub fn finish_get_fp(self) -> FP {
        let raw_hash = self.hasher.finish(); // u64

        FP::from_bits(u64_to_i64(raw_hash))
    }

    pub fn finish_get_point_on_unit_circle(self) -> FP2 {
        let raw_hash = self.hasher.finish();
        let angle = FP::from_bits(u64_to_i64(raw_hash)) % FP::from_num(2.0 * std::f64::consts::PI);
        FP2::new(angle.cos(), angle.sin())
    }

    // Hash a u32 value
    pub fn hash_u32(mut self, value: u32) -> Self {
        value.hash(&mut self.hasher);
        self
    }

    pub fn hash_id(mut self, value: Id) -> Self {
        ID_HASH_KEY.hash(&mut self.hasher);
        value.0.hash(&mut self.hasher);
        self
    }

    pub fn hash_net_id(mut self, value: NetId) -> Self {
        NET_ID_HASH_KEY.hash(&mut self.hasher);
        value.hash(&mut self.hasher);
        self
    }

    // Hash a FixedPoint value
    pub fn hash_fp(mut self, value: FP) -> Self {
        FIXED_POINT_HASH_KEY.hash(&mut self.hasher);
        value.hash(&mut self.hasher);
        self
    }

    pub fn hash_fp2(mut self, value: FP2) -> Self {
        FIXED_POINT_V2_HASH_KEY.hash(&mut self.hasher);
        value.hash(&mut self.hasher);
        self
    }

    pub fn hash_fp3(mut self, value: FP3) -> Self {
        FIXED_POINT_V3_HASH_KEY.hash(&mut self.hasher);
        value.hash(&mut self.hasher);
        self
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_u32() {
        let id_gen = RandomGen::start().hash_u32(42).finish_get_id();
        println!("Hash of u32 (42): {:?}", id_gen);
    }

    #[test]
    fn test_hash_id() {
        let id = Id::new(123);
        let id_gen = RandomGen::start().hash_id(id).finish_get_id();
        println!("Hash of Id (123): {:?}", id_gen);
    }

    #[test]
    fn test_hash_net_id() {
        let net_id = NetId::from_u32(456);
        let id_gen = RandomGen::start().hash_net_id(net_id).finish_get_id();
        println!("Hash of NetId (456): {:?}", id_gen);
    }

    #[test]
    fn test_hash_fp() {
        let fp = FP::from_num(3.14);
        let id_gen = RandomGen::start().hash_fp(fp).finish_get_id();
        println!("Hash of FixedPoint (3.14): {:?}", id_gen);
    }

    #[test]
    fn test_hash_fp2() {
        let fp2 = FP2::from_num(1.23, 4.56);
        let id_gen = RandomGen::start().hash_fp2(fp2).finish_get_id();
        println!("Hash of FixedPointV2 (1.23, 4.56): {:?}", id_gen);
    }

    #[test]
    fn test_hash_fp3() {
        let fp3 = FP3::from_num(7.89, 0.12, 3.45);
        let id_gen = RandomGen::start().hash_fp3(fp3).finish_get_id();
        println!("Hash of FixedPointV3 (7.89, 0.12, 3.45): {:?}", id_gen);
    }
}
