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
pub struct IdGeneratorBuilder {
    hasher: Blake3HasherWrapper,
}

impl IdGeneratorBuilder {
    // Initialize a new IdGeneratorBuilder
    pub fn start() -> Self {
        IdGeneratorBuilder {
            hasher: Blake3HasherWrapper(blake3::Hasher::new()),
        }
    }

    // Finish building and return the final hash
    pub fn finish(self) -> Id {
        let raw_hash = self.hasher.finish();
        Id::new(raw_hash as u32)
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
    pub fn hash_fp(mut self, value: FixedPoint) -> Self {
        FIXED_POINT_HASH_KEY.hash(&mut self.hasher);
        value.hash(&mut self.hasher);
        self
    }

    pub fn hash_fp2(mut self, value: FixedPointV2) -> Self {
        FIXED_POINT_V2_HASH_KEY.hash(&mut self.hasher);
        value.hash(&mut self.hasher);
        self
    }

    pub fn hash_fp3(mut self, value: FixedPointV3) -> Self {
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
        let id_gen = IdGeneratorBuilder::start().hash_u32(42).finish();
        println!("Hash of u32 (42): {:?}", id_gen);
    }

    #[test]
    fn test_hash_id() {
        let id = Id::new(123);
        let id_gen = IdGeneratorBuilder::start().hash_id(id).finish();
        println!("Hash of Id (123): {:?}", id_gen);
    }

    #[test]
    fn test_hash_net_id() {
        let net_id = NetId::from_u32(456);
        let id_gen = IdGeneratorBuilder::start().hash_net_id(net_id).finish();
        println!("Hash of NetId (456): {:?}", id_gen);
    }

    #[test]
    fn test_hash_fp() {
        let fp = FixedPoint::from_num(3.14);
        let id_gen = IdGeneratorBuilder::start().hash_fp(fp).finish();
        println!("Hash of FixedPoint (3.14): {:?}", id_gen);
    }

    #[test]
    fn test_hash_fp2() {
        let fp2 = FixedPointV2::from_num(1.23, 4.56);
        let id_gen = IdGeneratorBuilder::start().hash_fp2(fp2).finish();
        println!("Hash of FixedPointV2 (1.23, 4.56): {:?}", id_gen);
    }

    #[test]
    fn test_hash_fp3() {
        let fp3 = FixedPointV3::from_num(7.89, 0.12, 3.45);
        let id_gen = IdGeneratorBuilder::start().hash_fp3(fp3).finish();
        println!("Hash of FixedPointV3 (7.89, 0.12, 3.45): {:?}", id_gen);
    }
}
