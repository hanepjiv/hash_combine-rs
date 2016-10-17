// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/12
//  @date 2016/10/10

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![deny(missing_docs, dead_code, unused_imports, unused_variables)]
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use ::std::hash::{ Hasher, };
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// hash_combine
pub fn hash_combine(a_seed: u32, bytes: &[u8]) -> u32 {
    let mut seed = a_seed;
    for b in bytes {
        seed ^= (*b as u32) ^ 0x9e3779b9u32 ^ (seed << 6u32) ^ (seed >> 2u32) ;
    }
    seed
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct CombineHasher
#[derive( Debug, )]
pub struct CombineHasher {
    /// value
    value:      u32,
}
// ============================================================================
impl Default for CombineHasher {
    // ========================================================================
    fn default() -> Self { CombineHasher {
        value:  0u32,
    } }
}
// ============================================================================
impl CombineHasher {
    // ========================================================================
    /// new
    pub fn new(value: u32) -> Self { CombineHasher {
        value:  value,
    } }
}
// ============================================================================
impl Hasher for CombineHasher {
    // ========================================================================
    fn finish(&self) -> u64 { self.value as u64 }
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
    use ::std::hash::{ Hasher };
    // ------------------------------------------------------------------------
    use super::{ CombineHasher };
    // ========================================================================
    #[test]
    fn hash_combine() {
        let mut a = CombineHasher::default();
        a.write(&[
            0x2bu8, 0x6cu8, 0x81u8, 0x58u8,
            0xe8u8, 0x0fu8,
            0x11u8, 0xe5u8,
            0x82u8, 0xf7u8,
            0x00u8, 0x03u8, 0x0du8, 0x80u8, 0x79u8, 0x67u8
        ]);
        assert!(0x03d71136u32 == a.finish() as u32, "CombineHasher::finish");

        let mut a = CombineHasher::new(0u32);
        a.write(&[
            0x2bu8, 0x6cu8, 0x81u8, 0x58u8,
            0xe8u8, 0x0fu8,
            0x11u8, 0xe5u8,
            0x82u8, 0xf7u8,
            0x00u8, 0x03u8, 0x0du8, 0x80u8, 0x79u8, 0x67u8
        ]);
        assert!(0x03d71136u32 == a.finish() as u32, "CombineHasher::finish");
    }
}
