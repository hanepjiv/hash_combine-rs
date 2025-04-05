// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/12
//  @date 2025/04/06

#![cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
                                            "/README.md")))]
// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![no_std]
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// `hash_combine`
#[must_use]
#[inline]
pub fn hash_combine(a_seed: u32, bytes: &[u8]) -> u32 {
    let mut seed = a_seed;
    for b in bytes {
        seed ^= u32::from(*b)
            ^ 0x9e37_79b9_u32
            ^ (seed << 6_u32)
            ^ (seed >> 2_u32);
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
    #[inline]
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}
// ============================================================================
impl core::hash::Hasher for CombineHasher {
    // ========================================================================
    #[inline]
    fn finish(&self) -> u64 {
        u64::from(self.value)
    }
    // ========================================================================
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.value = hash_combine(self.value, bytes);
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[cfg(test)]
mod tests {
    // ========================================================================
    use core::hash::Hasher as _;
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
            0x2b_u8, 0x6c_u8, 0x81_u8, 0x58_u8, 0xe8_u8, 0x0f_u8, 0x11_u8,
            0xe5_u8, 0x82_u8, 0xf7_u8, 0x00_u8, 0x03_u8, 0x0d_u8, 0x80_u8,
            0x79_u8, 0x67_u8,
        ]);
        assert!(
            u64::from(0x03d7_1136_u32) == hasher.finish(),
            "CombineHasher::finish"
        );
    }
}
