use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let variants = match input.data {
        Data::Enum(data) => data.variants,
        _ => {
            return syn::Error::new_spanned(
                ident,
                "MaterialSet can only be derived for enums",
            )
            .to_compile_error()
            .into();
        }
    };

    let mut variant_idents = Vec::with_capacity(variants.len());
    let mut labels = Vec::with_capacity(variants.len());

    for variant in variants {
        if !matches!(variant.fields, Fields::Unit) {
            return syn::Error::new_spanned(
                variant,
                "MaterialSet can only be derived for unit enums",
            )
            .to_compile_error()
            .into();
        }

        let variant_ident = variant.ident;
        labels.push(variant_ident.to_string());
        variant_idents.push(variant_ident);
    }

    quote! {
        impl ::hyle_ca_interface::MaterialSet for #ident {
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
