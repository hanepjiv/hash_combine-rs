// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/12
//  @date 2018/07/25

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
// rustc 1.27.2 (58cc626de 2018-07-18)
#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    bare_trait_objects,
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
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    incoherent_fundamental_impls,
    late_bound_lifetime_arguments,
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
    safe_packed_borrows,
    stable_features,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unconditional_recursion,
    unions_with_drop_fields,
    unknown_lints,
    unreachable_code,
    unreachable_patterns,
    unstable_name_collisions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_comparisons,
    unused_doc_comments,
    unused_features,
    unused_imports,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_unsafe,
    unused_variables,
    while_true,
    exceeding_bitshifts,
    invalid_type_param_default,
    legacy_constructor_visibility,
    legacy_directory_ownership,
    legacy_imports,
    missing_fragment_specifier,
    mutable_transmutes,
    no_mangle_const_items,
    parenthesized_params_in_types_and_modules,
    pub_use_of_private_extern_crate,
    safe_extern_statics,
    unknown_crate_types,
)]
#![warn(dead_code, renamed_and_removed_lints, unreachable_pub)]
#![allow(
    box_pointers,
    elided_lifetimes_in_paths,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code
)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CombineHasher>();
    }
    // ------------------------------------------------------------------------
    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
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
            u64::from(0x03d71136u32) == hasher.finish(),
            "CombineHasher::finish"
        );
    }
}
