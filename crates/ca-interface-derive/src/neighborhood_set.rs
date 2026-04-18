use proc_macro::TokenStream;

use quote::quote;

use crate::common::parse_unit_enum;

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let parsed = match parse_unit_enum(input, "NeighborhoodSet") {
        Ok(parsed) => parsed,
        Err(error) => return error,
    };

    let ident = parsed.ident;
    let variant_idents = parsed
        .variants
        .iter()
        .map(|variant| variant.ident.clone())
        .collect::<Vec<_>>();
    let labels = parsed
        .variants
        .iter()
        .map(|variant| variant.label.clone())
        .collect::<Vec<_>>();

    quote! {
        impl ::hyle_ca_interface::NeighborhoodSet for #ident {
            fn variants() -> &'static [Self] {
                &[#(Self::#variant_idents),*]
            }

            fn label(self) -> &'static str {
                match self {
                    #(Self::#variant_idents => #labels),*
                }
            }
        }
    }
    .into()
}
