// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/12
//  @date 2020/03/19

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
// rustc 1.42.0 (b8cedc004 2020-03-09)
#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    box_pointers,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    indirect_structural_match,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    missing_doc_code_examples,
    non_ascii_idents,
    private_doc_tests,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    array_into_iter,
    bare_trait_objects,
    bindings_with_variant_name,
    deprecated,
    ellipsis_inclusive_range_patterns,
    exported_private_dependencies,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    incomplete_features,
    intra_doc_link_resolution_failure,
    invalid_value,
    irrefutable_let_patterns,
    late_bound_lifetime_arguments,
    mutable_borrow_reservation_conflict,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    no_mangle_generic_items,
    overlapping_patterns,
    path_statements,
    private_in_public,
    proc_macro_derive_resolution_fallback,
    redundant_semicolon,
    safe_packed_borrows,
    stable_features,
    trivial_bounds,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    uncommon_codepoints,
    unconditional_recursion,
    unknown_lints,
    unnameable_test_items,
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
    unused_labels,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_unsafe,
    unused_variables,
    where_clauses_object_safety,
    while_true,
    ambiguous_associated_items,
    conflicting_repr_hints,
    const_err,
    exceeding_bitshifts,
    ill_formed_attribute_input,
    invalid_type_param_default,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    missing_fragment_specifier,
    mutable_transmutes,
    no_mangle_const_items,
    order_dependent_trait_objects,
    overflowing_literals,
    patterns_in_fns_without_body,
    pub_use_of_private_extern_crate,
    soft_unstable,
    unknown_crate_types
)]
#![warn(dead_code, renamed_and_removed_lints)]
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
