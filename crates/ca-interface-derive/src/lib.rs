//! Proc-macro derives for enum-backed Hyle schema sets.
//!
//! These derives generate the small trait impls used by
//! `hyle_ca_interface::{MaterialSet, AttributeSet, NeighborhoodSet}` so
//! downstream enums can stay declarative.

mod attribute_set;
mod common;
mod material_set;
mod neighborhood_set;

use proc_macro::TokenStream;

/// Derive `hyle_ca_interface::MaterialSet` for a unit enum.
///
/// The generated impl preserves declaration order in `variants()` and uses the
/// variant name for `label()` unless a `#[label("...")]` override is present.
///
/// # Supported Attributes
///
/// - `#[label("name")]`: overrides the material label returned by
///   `MaterialSet::label()`.
///
/// # Errors
///
/// Emits a compile error when applied to a non-enum item or to an enum with
/// non-unit variants.
#[proc_macro_derive(MaterialSet, attributes(label))]
pub fn derive_material_set(input: TokenStream) -> TokenStream {
    material_set::derive(input)
}

/// Derive `hyle_ca_interface::AttributeSet` for a unit enum.
///
/// The generated impl preserves declaration order in `variants()`, uses the
/// variant name for `label()` unless a `#[label("...")]` override is present,
/// and maps each variant to an `hyle_ca_interface::AttributeType` through
/// `#[attribute_type(...)]`.
///
/// # Supported Attributes
///
/// - `#[label("name")]`: overrides the attribute label returned by
///   `AttributeSet::label()`.
/// - `#[attribute_type(Type)]`: sets the corresponding
///   `hyle_ca_interface::AttributeType` variant for
///   `AttributeSet::value_type()`.
///
/// # Errors
///
/// Emits a compile error when applied to a non-enum item, to an enum with
/// non-unit variants, or when any variant omits `#[attribute_type(...)]`.
#[proc_macro_derive(AttributeSet, attributes(label, attribute_type))]
pub fn derive_attribute_set(input: TokenStream) -> TokenStream {
    attribute_set::derive(input)
}

/// Derive `hyle_ca_interface::NeighborhoodSet` for a unit enum.
///
/// The generated impl preserves declaration order in `variants()` and uses the
/// variant name for `label()` unless a `#[label("...")]` override is present.
///
/// # Supported Attributes
///
/// - `#[label("name")]`: overrides the neighborhood label returned by
///   `NeighborhoodSet::label()`.
///
/// # Errors
///
/// Emits a compile error when applied to a non-enum item or to an enum with
/// non-unit variants.
#[proc_macro_derive(NeighborhoodSet, attributes(label))]
pub fn derive_neighborhood_set(input: TokenStream) -> TokenStream {
    neighborhood_set::derive(input)
}
