use proc_macro2::TokenStream;
use syn::spanned::Spanned;

use crate::model::Microtype;

use self::{
    errors::{secret_feature_missing, serialize_without_serde},
    special_attrs::{strip_special_attrs, SecretAttr},
};

mod diesel;
mod normal;
mod secret;

mod errors;
mod special_attrs;

const HAS_SERDE: bool = cfg!(feature = "serde");
const HAS_TEST_IMPLS: bool = cfg!(feature = "test_impls");
const HAS_DEREF_IMPLS: bool = cfg!(feature = "deref_impls");
const HAS_SECRET: bool = cfg!(feature = "secret");

pub fn codegen(microtypes: Vec<Microtype>) -> TokenStream {
    let mut stream = TokenStream::new();

    for microtype in microtypes {
        let tokens = generate_single(microtype);
        stream.extend(tokens);
    }

    stream
}

fn generate_single(
    Microtype {
        inner,
        name,
        attrs,
        vis,
    }: Microtype,
) -> TokenStream {
    let (attrs, special_attrs) = match strip_special_attrs(attrs) {
        Ok(ok) => ok,
        Err(tokens) => return tokens,
    };

    if !HAS_SERDE {
        if let Some(SecretAttr {
            serialize: Some(_),
            path,
        }) = special_attrs.secret
        {
            return serialize_without_serde(path.span());
        }
    }

    match &special_attrs.secret {
        None => normal::generate_normal(inner, name, vis, attrs, special_attrs),
        Some(SecretAttr { path, .. }) => {
            if HAS_SECRET {
                secret::generate_secret(inner, name, attrs, vis, special_attrs)
            } else {
                secret_feature_missing(path.span())
            }
        }
    }
}

