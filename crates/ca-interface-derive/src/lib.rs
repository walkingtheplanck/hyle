mod attribute_set;
mod common;
mod material_set;
mod neighborhood_set;

use proc_macro::TokenStream;

#[proc_macro_derive(MaterialSet, attributes(label))]
pub fn derive_material_set(input: TokenStream) -> TokenStream {
    material_set::derive(input)
}

#[proc_macro_derive(AttributeSet, attributes(label, attribute_type))]
pub fn derive_attribute_set(input: TokenStream) -> TokenStream {
    attribute_set::derive(input)
}

#[proc_macro_derive(NeighborhoodSet, attributes(label))]
pub fn derive_neighborhood_set(input: TokenStream) -> TokenStream {
    neighborhood_set::derive(input)
}
