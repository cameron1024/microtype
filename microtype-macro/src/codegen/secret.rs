use crate::codegen::special_attrs::TypeAnnotation;

use super::{special_attrs::SpecialAttrs, HAS_SERDE, HAS_TEST_IMPLS};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Ident, Type, Visibility};

fn attrs_for_both(serialize: bool) -> TokenStream {
    let mut attrs = quote! {
        #[repr(transparent)]
        #[derive(::std::clone::Clone)]
        #[cfg_attr(not(test), derive(::std::fmt::Debug))]
    };

    // without this feature, we just derive debug in test builds as well
    if !HAS_TEST_IMPLS {
        attrs.extend(quote! {
            #[cfg_attr(test, derive(::std::fmt::Debug))]
        });
    }

    if HAS_SERDE {
        attrs.extend(match serialize {
            false => quote! {
                #[derive(::serde::Deserialize)]
            },
            true => quote! {
                #[derive(::serde::Deserialize)]
                #[derive(::serde::Serialize)]
            },
        })
    }

    attrs
}

fn test_impls(name: &Ident) -> TokenStream {
    quote! {
        #[cfg(test)]
        impl ::std::fmt::Debug for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use ::microtype::secrecy::ExposeSecret;
                f.write_str(self.expose_secret())
            }
        }

        #[cfg(test)]
        impl ::std::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                use ::microtype::secrecy::ExposeSecret;
                self.expose_secret().eq(other.expose_secret())
            }
        }
    }
}

fn wrapper_impls(serialize: bool, wrapper: &Ident) -> TokenStream {
    let mut tokens = quote! {
        impl ::microtype::secrecy::CloneableSecret for #wrapper {}
        impl ::microtype::secrecy::DebugSecret for #wrapper {}
        impl ::microtype::secrecy::Zeroize for #wrapper {
            fn zeroize(&mut self) {
                ::microtype::secrecy::Zeroize::zeroize(&mut self.0)
            }
        }
    };

    if serialize && HAS_SERDE {
        tokens.extend(quote! {
            impl ::microtype::secrecy::SerializableSecret for #wrapper {}
        });
    }

    tokens
}

fn expose_secret_impl(name: &Ident, inner: &Type) -> TokenStream {
    quote! {
        impl ::microtype::secrecy::ExposeSecret<#inner> for #name {
            fn expose_secret(&self) -> &#inner {
                use ::microtype::secrecy::ExposeSecret;
                &self.0.expose_secret().0
            }
        }
    }
}

fn secret_microtype_impl(name: &Ident, wrapper: &Ident, inner: &Type) -> TokenStream {
    quote! {
        impl ::microtype::SecretMicrotype for #name {
            type Inner = #inner;

            fn new(inner: Self::Inner) -> Self {
                Self(::microtype::secrecy::Secret::new(#wrapper(inner)))
            }
        }
    }
}

fn generate_structs(
    name: &Ident,
    inner: &Type,
    vis: &Visibility,
    extra_attrs: &[Attribute],
    serialize: bool,
) -> (TokenStream, Ident) {
    let wrapper = Ident::new(&format!("__Wrapper{}", name), name.span());
    let attrs_for_both = attrs_for_both(serialize);

    let tokens = quote! {
        #(#extra_attrs)*
        #attrs_for_both
        #vis struct #name(::microtype::secrecy::Secret<#wrapper>);

        #attrs_for_both
        struct #wrapper(#inner);
    };

    (tokens, wrapper)
}

fn string_impls(name: &Ident) -> TokenStream {
    quote! {
        impl ::core::str::FromStr for #name {
            type Err = ::std::convert::Infallible;

            fn from_str(s: &::core::primitive::str) -> Result<Self, Self::Err> {
                Ok(<Self as ::microtype::SecretMicrotype>::new(s.to_string()))
            }
        }

        impl ::core::convert::AsRef<::core::primtitive::str> for #name {
            fn as_ref(&self) -> &::core::primitive::str {
                &self.0
            }
        }
    }
}

pub fn generate_secret(
    inner: Type,
    name: Ident,
    extra_attrs: Vec<Attribute>,
    vis: Visibility,
    special_attrs: SpecialAttrs,
) -> TokenStream {
    assert!(
        special_attrs.secret.is_some(),
        "we are generating a secret type, so `secret` must be `Some`"
    );
    let secret = special_attrs.secret.unwrap();
    let serialize = secret.serialize.is_some();

    let (struct_defs, wrapper) = generate_structs(&name, &inner, &vis, &extra_attrs, serialize);
    let wrapper_impls = wrapper_impls(serialize, &wrapper);
    let test_impls = test_impls(&name);
    let expose_secret_impl = expose_secret_impl(&name, &inner);
    let secret_microtype_impl = secret_microtype_impl(&name, &wrapper, &inner);

    let type_specific_impls = match special_attrs.type_annotation {
        None => quote! {},
        Some(TypeAnnotation::String) => string_impls(&name),
        Some(TypeAnnotation::Int) => todo!(),
    };

    quote! {
        #struct_defs

        #wrapper_impls
        #expose_secret_impl
        #secret_microtype_impl
        #test_impls
        #type_specific_impls
    }
}
