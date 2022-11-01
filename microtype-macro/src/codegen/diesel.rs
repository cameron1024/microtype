use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

use super::HAS_DIESEL;

pub fn diesel_impl_not_secret(sql_type: &Type, inner: &Type, name: &Ident) -> TokenStream {
    let from_sql = from_sql_not_secret(sql_type, inner, name);
    let to_sql = to_sql_not_secret(sql_type, inner, name);

    if HAS_DIESEL {
        quote! {
            #from_sql
            #to_sql
        }
    } else {
        quote! {}
    }
}

pub fn diesel_impl_secret(sql_type: &Type, inner: &Type, name: &Ident) -> TokenStream {
    let from_sql = from_sql_secret(sql_type, inner, name);
    let to_sql = to_sql_secret(sql_type, inner, name);

    if HAS_DIESEL {
        quote! {
            #from_sql
            #to_sql
        }
    } else {
        quote! {}
    }
}

fn from_sql_not_secret(sql_type: &Type, inner: &Type, name: &Ident) -> TokenStream {
    quote! {
        impl<B: ::diesel::backend::Backend> ::diesel::deserialize::FromSql<#sql_type, B> for #name
        where
            #inner: ::diesel::deserialize::FromSql<#sql_type, B>,
        {
            fn from_sql(
                bytes: ::diesel::backend::RawValue<'_, B>,
            ) -> ::diesel::deserialize::Result<Self> {
                <#inner as ::diesel::deserialize::FromSql<#sql_type, B>>::from_sql(bytes).map(#name)
            }
        }
    }
}

fn to_sql_not_secret(sql_type: &Type, inner: &Type, name: &Ident) -> TokenStream {
    quote! {
        impl<B: ::diesel::backend::Backend> ::diesel::serialize::ToSql<#sql_type, B> for #name
        where
            #inner: ::diesel::serialize::ToSql<#sql_type, B>,
        {
            fn to_sql<'b>(
                &'b self,
                out: &mut diesel::serialize::Output<'b, '_, B>,
            ) -> diesel::serialize::Result {
                <#inner as ::diesel::serialize::ToSql<#sql_type, B>>::to_sql(&self.0, out)
            }
        }

    }
}

fn from_sql_secret(sql_type: &Type, inner: &Type, name: &Ident) -> TokenStream {
    quote! {
        impl<B: ::diesel::backend::Backend> ::diesel::deserialize::FromSql<#sql_type, B> for #name
        where
            #inner: ::diesel::deserialize::FromSql<#sql_type, B>,
        {
            fn from_sql(bytes: ::diesel::backend::RawValue<'_, B>) -> ::diesel::deserialize::Result<Self> {
                <#inner as ::diesel::deserialize::FromSql<#sql_type, B>>::from_sql(bytes)
                    .map(<Self as ::microtype::SecretMicrotype>::new)
            }
        }

    }
}

fn to_sql_secret(sql_type: &Type, inner: &Type, name: &Ident) -> TokenStream {
    quote! {
        impl<B: ::diesel::backend::Backend> ::diesel::serialize::ToSql<#sql_type, B> for #name
        where
            #inner: ::diesel::serialize::ToSql<#sql_type, B>,
        {
            fn to_sql<'b>(
                &'b self,
                out: &mut diesel::serialize::Output<'b, '_, B>,
            ) -> diesel::serialize::Result {
                <#inner as ::diesel::serialize::ToSql<#sql_type, B>>::to_sql(
                    <Self as ::microtype::secrecy::ExposeSecret<#inner>>::expose_secret(&self),
                    out,
                )
            }
        }
    }
}
