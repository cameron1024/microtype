use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Path, Type};

pub fn fmt_impl(name: &Ident, inner: &Type, trait_name: &Path) -> TokenStream {
    quote! {
        impl #trait_name for #name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <#inner as #trait_name>::fmt(&self.0, f)
            }
        }
    }
}
