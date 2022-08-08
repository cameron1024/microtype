use super::{special_attrs::SpecialAttrs, HAS_DEREF_IMPLS, HAS_SERDE};
use quote::quote;
use syn::{Attribute, Ident, Type, Visibility};

fn generate_struct(name: &Ident, vis: &Visibility, inner: &Type) -> proc_macro2::TokenStream {
    quote! {
        #[repr(transparent)]
        #vis struct #name(pub #inner);
    }
}

fn generate_microtype_impl(name: &Ident, inner: &Type) -> proc_macro2::TokenStream {
    quote! {
        impl ::microtype::Microtype for #name {
            type Inner = #inner;

            fn new(inner: Self::Inner) -> Self {
                Self(inner)
            }

            fn into_inner(self) -> Self::Inner {
                self.0
            }

            fn inner(&self) -> &Self::Inner {
                &self.0
            }

            fn inner_mut(&mut self) -> &mut Self::Inner {
                &mut self.0
            }


            fn convert<T: ::microtype::Microtype<Inner = Self::Inner>>(self) -> T {
                T::new(self.0)
            }
        }
    }
}

fn generate_from_impl(name: &Ident, inner: &Type) -> proc_macro2::TokenStream {
    quote! {
        impl ::std::convert::From<#inner> for #name {
            fn from(inner: #inner) -> Self {
                Self(inner)
            }
        }
    }
}

fn generate_deref_impl(name: &Ident, inner: &Type) -> proc_macro2::TokenStream {
    if HAS_DEREF_IMPLS {
        quote! {
            impl ::std::ops::Deref for #name {
                type Target = #inner;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        }
    } else {
        quote! {}
    }
}

fn serde_derives() -> proc_macro2::TokenStream {
    if HAS_SERDE {
        quote! {
            #[derive(::serde::Deserialize, ::serde::Serialize)]
            #[serde(transparent)]
        }
    } else {
        quote! {}
    }
}

pub fn generate_normal(
    inner: Type,
    name: Ident,
    vis: Visibility,
    attrs: Vec<Attribute>,
    _special_attrs: SpecialAttrs,
) -> proc_macro2::TokenStream {
    let struct_def = generate_struct(&name, &vis, &inner);
    let microtype_impl = generate_microtype_impl(&name, &inner);
    let from_impl = generate_from_impl(&name, &inner);
    let deref_impl = generate_deref_impl(&name, &inner);
    let serde_attrs = serde_derives();

    quote! {
        #(#attrs)*
        #serde_attrs
        #struct_def

        #microtype_impl

        #from_impl
        #deref_impl

    }
}
