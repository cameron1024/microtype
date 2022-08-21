use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_str, Ident, Type};

use super::helpers::fmt_impl;

pub fn generate_string_impls(name: &Ident, inner: &Type) -> TokenStream {
    let display = fmt_impl(name, inner, &parse_str("::core::fmt::Display").unwrap());

    quote! {

        #display

        impl ::core::str::FromStr for #name {
            type Err = ::core::convert::Infallible;

            fn from_str(s: &::core::primitive::str) -> Result<Self, Self::Err> {
                Ok(Self(s.to_string()))
            }

        }

        impl ::std::convert::From<&::core::primitive::str> for #name {
            fn from(s: &::core::primitive::str) -> Self {
                Self::from(s.to_string())
            }
        }

        impl ::core::convert::AsRef<::core::primitive::str> for #name {
            fn as_ref(&self) -> &::core::primitive::str {
                &self.0
            }
        }
    }
}
