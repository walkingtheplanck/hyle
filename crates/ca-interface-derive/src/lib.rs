mod material_set;

use proc_macro::TokenStream;

#[proc_macro_derive(MaterialSet)]
pub fn derive_material_set(input: TokenStream) -> TokenStream {
    material_set::derive(input)
}
