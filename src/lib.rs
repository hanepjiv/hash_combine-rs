// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/12
//  @date 2017/02/23

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![deny(
    fat_ptr_transmutes,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    const_err,
    deprecated,
    deprecated_attr,
    extra_requirement_in_impl,
    improper_ctypes,
    legacy_imports,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    private_no_mangle_fns,
    private_no_mangle_statics,
    renamed_and_removed_lints,
    safe_extern_statics,
    stable_features,
    unconditional_recursion,
    unions_with_drop_fields,
    unknown_lints,
    unreachable_code,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_comparisons,
    unused_features,
    unused_imports,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_unsafe,
    unused_variables,
    while_true,
    exceeding_bitshifts,
    hr_lifetime_in_assoc_type,
    illegal_floating_point_constant_pattern,
    illegal_struct_or_enum_constant_pattern,
    inaccessible_extern_crate,
    invalid_type_param_default,
    lifetime_underscore,
    mutable_transmutes,
    no_mangle_const_items,
    overlapping_inherent_impls,
    super_or_self_in_global_path,
    transmute_from_fn_item_types,
    unknown_crate_types,
)]
#![warn(
    dead_code,
)]
#![allow(
    box_pointers,
    unsafe_code,
    trivial_casts,
    trivial_numeric_casts,
)]
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use ::std::hash::Hasher;
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
#[derive( Debug, Clone, Copy, )]
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
    use ::std::hash::Hasher;
    // ------------------------------------------------------------------------
    use super::CombineHasher;
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
