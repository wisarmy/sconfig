use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
/// derive toml
///
/// # Attributes
///
/// The following attributes are supported:
///
/// - `#[toml(inline)]`: Specifies that the struct or enum should be
///   serialized inline, without wrapping it in a table. This attribute
///   only applies to structs or enums that have named fields.
#[proc_macro_derive(Toml, attributes(toml))]
pub fn derive_toml(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let use_inline = input
        .attrs
        .iter()
        .any(|attr| attr.path.is_ident("toml") && attr.tokens.to_string() == "(inline)");
    let expaned = if use_inline {
        quote! {
            impl std::str::FromStr for #name {
                type Err = toml::de::Error;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    toml::from_str(s)
                }
            }
            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match toml_edit::ser::to_string(self) {
                        Ok(v) => {
                             write!(f, "{}", v)
                        },
                        Err(e) => {
                            tracing::error!("{}", e) ;
                            Err(std::fmt::Error::default())
                        }
                    }
                }
            }
        }
    } else {
        quote! {
            impl std::str::FromStr for #name {
                type Err = toml::de::Error;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    toml::from_str(s)
                }
            }
            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match toml::to_string(self) {
                        Ok(v) => {
                             write!(f, "{}", v)
                        },
                        Err(e) => {
                            tracing::error!("{}", e) ;
                            Err(std::fmt::Error::default())
                        }
                    }
                }
            }
        }
    };
    TokenStream::from(expaned)
}
#[proc_macro_derive(Json)]
pub fn derive_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expaned = quote! {
        impl std::str::FromStr for #name {
            type Err = serde_json::Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                serde_json::from_str(s)
            }
        }
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match serde_json::to_string_pretty(self) {
                    Ok(v) => {
                         write!(f, "{}", v)
                    },
                    Err(e) => {
                        tracing::error!("{}", e) ;
                        Err(std::fmt::Error::default())
                    }
                }
            }
        }
    };
    TokenStream::from(expaned)
}
