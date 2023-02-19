use proc_macro::{TokenStream};
use std::fmt::format;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{Expr, Ident, Macro, parse_macro_input, parse_str, Type};
use syn::parse::{Parse, ParseStream};

#[proc_macro]
pub fn impl_into_similar(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ImplIntoSimilarMacroInput);
    let from = input.from;
    let into = input.into;

    let output = quote! {
        impl TryInto<#into> for #from {
            type Error = serde_json::Error;

            fn try_into(self) -> Result<#into, Self::Error> {
                let serialized = serde_json::to_string(&self)?;

                serde_json::from_str(&serialized)
            }
        }
    };

    TokenStream::from(output)
}

struct ImplIntoSimilarMacroInput {
    into: Type,
    from: Type
}

impl Parse for ImplIntoSimilarMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let into = Type::parse(input)?;
        let _: syn::token::Comma = input.parse()?;
        let from = Type::parse(input)?;

        Ok(Self {
            into,
            from
        })
    }
}