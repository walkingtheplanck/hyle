use proc_macro::TokenStream;

use quote::quote;

use crate::common::{parse_attribute_type, parse_unit_enum};

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let parsed = match parse_unit_enum(input, "AttributeSet") {
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
    let value_types = match parsed
        .variants
        .iter()
        .map(|variant| parse_attribute_type(&variant.attrs))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(value_types) => value_types,
        Err(error) => return error.to_compile_error().into(),
    };

    quote! {
        impl ::hyle_ca_interface::AttributeSet for #ident {
            fn variants() -> &'static [Self] {
                &[#(Self::#variant_idents),*]
            }

            fn label(self) -> &'static str {
                match self {
                    #(Self::#variant_idents => #labels),*
                }
            }

            fn value_type(self) -> ::hyle_ca_interface::AttributeType {
                match self {
                    #(Self::#variant_idents => ::hyle_ca_interface::AttributeType::#value_types),*
                }
            }
        }
    }
    .into()
}
