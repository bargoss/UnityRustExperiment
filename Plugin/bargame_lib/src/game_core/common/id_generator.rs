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
