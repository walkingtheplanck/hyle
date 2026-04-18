use proc_macro::TokenStream;

use syn::{Attribute, Data, DeriveInput, Fields, Ident, LitStr};

pub(crate) struct ParsedEnum {
    pub(crate) ident: Ident,
    pub(crate) variants: Vec<ParsedVariant>,
}

pub(crate) struct ParsedVariant {
    pub(crate) ident: Ident,
    pub(crate) label: LitStr,
    pub(crate) attrs: Vec<Attribute>,
}

pub(crate) fn parse_unit_enum(
    input: TokenStream,
    derive_name: &str,
) -> Result<ParsedEnum, TokenStream> {
    let input = match syn::parse::<DeriveInput>(input) {
        Ok(input) => input,
        Err(error) => return Err(error.to_compile_error().into()),
    };
    let ident = input.ident;

    let variants = match input.data {
        Data::Enum(data) => data.variants,
        _ => {
            return Err(syn::Error::new_spanned(
                &ident,
                format!("{derive_name} can only be derived for enums"),
            )
            .to_compile_error()
            .into())
        }
    };

    let mut parsed_variants = Vec::with_capacity(variants.len());

    for variant in variants {
        if !matches!(variant.fields, Fields::Unit) {
            return Err(syn::Error::new_spanned(
                &variant,
                format!("{derive_name} can only be derived for unit enums"),
            )
            .to_compile_error()
            .into());
        }

        let variant_ident = variant.ident;
        let label = parse_label(&variant.attrs)
            .unwrap_or_else(|| LitStr::new(&variant_ident.to_string(), variant_ident.span()));
        parsed_variants.push(ParsedVariant {
            ident: variant_ident,
            label,
            attrs: variant.attrs,
        });
    }

    Ok(ParsedEnum {
        ident,
        variants: parsed_variants,
    })
}

fn parse_label(attrs: &[Attribute]) -> Option<LitStr> {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("label"))
        .and_then(|attr| attr.parse_args::<LitStr>().ok())
}

pub(crate) fn parse_attribute_type(attrs: &[Attribute]) -> Result<Ident, syn::Error> {
    for attr in attrs {
        if attr.path().is_ident("attribute_type") {
            return attr.parse_args::<Ident>();
        }
    }

    Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "AttributeSet variants must declare #[attribute_type(...)]",
    ))
}
