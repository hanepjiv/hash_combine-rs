// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/12
//  @date 2025/03/01

#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![no_std]
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// `hash_combine`
#[must_use]
pub fn hash_combine(a_seed: u32, bytes: &[u8]) -> u32 {
    let mut seed = a_seed;
    for b in bytes {
        seed ^=
            u32::from(*b) ^ 0x9e37_79b9u32 ^ (seed << 6u32) ^ (seed >> 2u32);
    }
    seed
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct `CombineHasher`
#[derive(
    Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct CombineHasher {
    /// value
    value: u32,
}
// ============================================================================
impl CombineHasher {
    // ========================================================================
    /// new
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}
// ============================================================================
impl core::hash::Hasher for CombineHasher {
    // ========================================================================
    fn finish(&self) -> u64 {
        u64::from(self.value)
    }
    // ========================================================================
    fn write(&mut self, bytes: &[u8]) {
        self.value = hash_combine(self.value, bytes);
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[cfg(test)]
mod tests {
    // ========================================================================
    use core::hash::Hasher;
    // ------------------------------------------------------------------------
    use super::CombineHasher;
    // ========================================================================
    #[test]
    const fn test_send() {
        const fn assert_send<T: Send>() {}
        assert_send::<CombineHasher>();
    }
    // ------------------------------------------------------------------------
    #[test]
    const fn test_sync() {
        const fn assert_sync<T: Sync>() {}
        assert_sync::<CombineHasher>();
    }
    // ========================================================================
    #[test]
    fn hash_combine() {
        let mut hasher = CombineHasher::default();
        hasher.write(&[
            0x2bu8, 0x6cu8, 0x81u8, 0x58u8, 0xe8u8, 0x0fu8, 0x11u8, 0xe5u8,
            0x82u8, 0xf7u8, 0x00u8, 0x03u8, 0x0du8, 0x80u8, 0x79u8, 0x67u8,
        ]);
        assert!(
            u64::from(0x03d7_1136_u32) == hasher.finish(),
            "CombineHasher::finish"
        );
    }
}
