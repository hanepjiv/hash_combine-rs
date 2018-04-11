// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/12
//  @date 2018/04/10

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![deny(
    anonymous_parameters, box_pointers, missing_copy_implementations,
    missing_debug_implementations, missing_docs, trivial_casts,
    trivial_numeric_casts, unsafe_code, unstable_features,
    unused_extern_crates, unused_import_braces, unused_qualifications,
    unused_results, variant_size_differences, const_err, dead_code, deprecated,
    illegal_floating_point_literal_pattern, improper_ctypes,
    late_bound_lifetime_arguments, non_camel_case_types,
    non_shorthand_field_patterns, non_snake_case, non_upper_case_globals,
    no_mangle_generic_items, overflowing_literals, path_statements,
    patterns_in_fns_without_body, plugin_as_library, private_in_public,
    private_no_mangle_fns, private_no_mangle_statics,
    renamed_and_removed_lints, stable_features, unconditional_recursion,
    unions_with_drop_fields, unknown_lints, unreachable_code,
    unreachable_patterns, unused_allocation, unused_assignments,
    unused_attributes, unused_comparisons, unused_doc_comment, unused_features,
    unused_imports, unused_macros, unused_must_use, unused_mut, unused_parens,
    unused_unsafe, unused_variables, while_true
)]
#![warn(dead_code)]
#![allow(box_pointers, unsafe_code, trivial_casts, trivial_numeric_casts)]
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::hash::Hasher;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// hash_combine
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
/// struct CombineHasher
#[derive(Debug, Clone, Copy)]
pub struct CombineHasher {
    /// value
    value: u32,
}
// ============================================================================
impl Default for CombineHasher {
    // ========================================================================
    fn default() -> Self {
        CombineHasher { value: 0u32 }
    }
}
// ============================================================================
impl CombineHasher {
    // ========================================================================
    /// new
    pub fn new(value: u32) -> Self {
        CombineHasher { value }
    }
}
// ============================================================================
impl Hasher for CombineHasher {
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
    use std::hash::Hasher;
    // ------------------------------------------------------------------------
    use super::CombineHasher;
    // ========================================================================
    #[test]
    fn hash_combine() {
        let mut a = CombineHasher::default();
        a.write(&[
            0x2bu8, 0x6cu8, 0x81u8, 0x58u8, 0xe8u8, 0x0fu8, 0x11u8, 0xe5u8,
            0x82u8, 0xf7u8, 0x00u8, 0x03u8, 0x0du8, 0x80u8, 0x79u8, 0x67u8,
        ]);
        assert!(
            0x03d71136u32 == a.finish() as u32,
            "CombineHasher::finish"
        );
    }
}
