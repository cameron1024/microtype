use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;

pub fn serialize_without_serde(span: Span) -> TokenStream {
    quote_spanned! {
        span => compile_error!("`#[secret(serialize)]` has no effect unless the `serde_support` feature is enabled]")
    }
}

pub fn secret_feature_missing(span: Span) -> TokenStream {
    quote_spanned! {
        span => compile_error!("`#[secret] is only supported when the `secret` feature is enabled")
    }
}
