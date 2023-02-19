use proc_macro::{TokenStream};
use quote::{quote};
use syn::{parse_macro_input, Type};
use syn::parse::{Parse, ParseStream};

#[proc_macro]
pub fn impl_try_from_similar(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ImplIntoSimilarMacroInput);
    let from = input.from;
    let into = input.into;

    let output = quote! {
        impl TryFrom<#from> for #into {
            type Error = anyhow::Error;

            fn try_from(from: #from) -> Result<#into, Self::Error> {
                let serialized = serde_json::to_string(&from)?;

                serde_json::from_str(&serialized)
                    .map_err(|e| anyhow::anyhow!("Failed to covert between types"))
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