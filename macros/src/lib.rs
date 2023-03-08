use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Toml)]
pub fn derive_toml(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expaned = quote! {
        use config::FileType;
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
    };
    TokenStream::from(expaned)
}
#[proc_macro_derive(Json)]
pub fn derive_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expaned = quote! {
        use config::FileType;
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
