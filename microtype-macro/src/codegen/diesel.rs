use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Ident, Type};

use crate::parse::DieselTypeAttr;

#[allow(dead_code)]
fn parse_diesel_attr(attr: &Attribute) -> Option<Ident> {
    if attr.path.get_ident().map(Ident::to_string) == Some("sql_type".into()) {
        match attr.parse_args::<DieselTypeAttr>() {
            Ok(DieselTypeAttr { ty }) => Some(ty),
            _ => None,
        }
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn generate_diesel_impls(
    inner: Type,
    name: Ident,
    attrs: &[Attribute],
    secret: bool,
) -> TokenStream {
    let new = if secret {
        quote! {
            <#name as ::microtype::SecretMicrotype>::new
        }
    } else {
        quote! {
            <#name as ::microtype::Microtype>::new
        }
    };

    let from = if secret {
        quote! {
            <#name as ::microtype::secrecy::ExposeSecret<#inner>>::expose_secret(self)
        }
    } else {
        quote! {
            <#name as ::microtype::Microtype>::inner(self)
        }
    };


    let attr = attrs.iter().find_map(parse_diesel_attr);

    match attr {
        None => quote! {},
        Some(diesel_type) => {
            quote! {
                impl<B: ::diesel::backend::Backend> ::diesel::types::FromSql<#diesel_type, B> for #name {
                    fn from_sql(bytes: ::std::option::Option<&B::RawValue>) -> ::diesel::deserialize::Result<Self> {
                        <#inner as ::diesel::types::FromSql<$diesel, B>::from_sql(bytes).map(#new)
                    }
                }

                impl<B: ::diesel::backend::Backend> ::diesel::types::ToSql<#diesel_type, B> for #name {
                    fn to_sql<W: ::std::io::Write>(
                            &self,
                            out: &mut ::diesel::serialize::Output<W, B>,
                        ) -> ::diesel::serialize::Result {

                        <#inner as ToSql<#diesel_type, B>::to_sql(
                            #from
                            out,
                        )

                    }
                }
            }
        }
    }
}
