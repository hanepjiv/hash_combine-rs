/* -*- mode:rust; coding:utf-8-unix; -*- */

// @author hanepjiv <hanepjiv@gmail.com>
// @since 2016/03/12
// @date 2016/05/08

// The MIT License (MIT)
//
// Copyright (c) <2016> hanepjiv <hanepjiv@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! lib.rs

/* ////////////////////////////////////////////////////////////////////////// */
/* attribute  =============================================================== */
#![deny(missing_docs, dead_code, unused_imports, unused_variables)]
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
use ::std::hash::{ Hasher, };
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// hash_combine
pub fn hash_combine(a_seed: u32, bytes: &[u8]) -> u32 {
    let mut seed = a_seed;
    for b in bytes {
        seed ^= (*b as u32) ^ 0x9e3779b9u32 ^ (seed << 6u32) ^ (seed >> 2u32) ;
    }
    seed
}
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// struct CombineHasher
#[derive( Debug, )]
pub struct CombineHasher {
    /// value
    value:      u32,
}
/* ========================================================================== */
impl Default for CombineHasher {
    /* ====================================================================== */
    fn default() -> Self { CombineHasher {
        value:  0u32,
    } }
}
/* ========================================================================== */
impl CombineHasher {
    /* ====================================================================== */
    /// new
    pub fn new(value: u32) -> Self { CombineHasher {
        value:  value,
    } }
}
/* ========================================================================== */
impl Hasher for CombineHasher {
    /* ====================================================================== */
    fn finish(&self) -> u64 { self.value as u64 }
    /* ====================================================================== */
    fn write(&mut self, bytes: &[u8]) {
        self.value = hash_combine(self.value, bytes);
    }
}
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
#[cfg(test)]
mod tests {
    /* ====================================================================== */
    use ::std::hash::{ Hasher };
    /* ---------------------------------------------------------------------- */
    use super::{ CombineHasher };
    /* ====================================================================== */
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
