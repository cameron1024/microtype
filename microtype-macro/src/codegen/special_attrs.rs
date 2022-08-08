use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use syn::{spanned::Spanned, Attribute, Ident, Meta, NestedMeta, Path};

fn generic_err(span: Span) -> TokenStream {
    quote_spanned!(span => compile_error!("expected either `#[secret]` or `#[secret(serialize)]`"))
}

fn duplicate_secret(span: Span) -> TokenStream {
    quote_spanned!(span => compile_error!("duplicate `secret` attribute found"))
}

fn duplicate_string(span: Span) -> TokenStream {
    quote_spanned!(span => compile_error!("duplicate `string` attribute found"))
}

pub fn strip_special_attrs(
    attrs: Vec<Attribute>,
) -> Result<(Vec<Attribute>, SpecialAttrs), TokenStream> {
    let (secret, attrs): (Vec<_>, Vec<_>) = attrs
        .into_iter()
        .partition(|attr| attr.path.is_ident("secret"));

    let secret = match &secret[..] {
        [] => None,
        [_first, second, ..] => return Err(duplicate_secret(second.span())),
        [single] => {
            let secret_attr = match single.parse_meta() {
                Ok(Meta::List(list)) => {
                    let nested: Vec<_> = list.nested.iter().collect();
                    let serialize = match &nested[..] {
                        // it's just `#[secret]`
                        [] => None,
                        // `#[secret(serialize)]`
                        [NestedMeta::Meta(Meta::Path(path))] if path.is_ident("serialize") => {
                            let ident = Ident::new("serialize", path.span());
                            Some(ident)
                        }
                        // anything else
                        [other, ..] => return Err(generic_err(other.span())),
                    };

                    let path = single.path.clone();

                    SecretAttr { path, serialize }
                }
                Ok(Meta::Path(path)) => SecretAttr {
                    path,
                    serialize: None,
                },
                Ok(other) => {
                    println!("other: {other:?}");
                    return Err(generic_err(other.span()));
                }
                Err(e) => return Err(e.to_compile_error()),
            };

            Some(secret_attr)
        }
    };

    let (string, attrs): (Vec<_>, Vec<_>) = attrs
        .into_iter()
        .partition(|attr| attr.path.is_ident("string"));

    let string = match &string[..] {
        [] => false,
        [_single] => true,
        [_, second, ..] => return Err(duplicate_string(second.span())),
    };

    let special_attrs = SpecialAttrs { secret, string };

    Ok((attrs, special_attrs))
}

pub struct SpecialAttrs {
    pub secret: Option<SecretAttr>,
    pub string: bool,
}

pub struct SecretAttr {
    pub serialize: Option<Ident>,
    pub path: Path,
}

#[cfg(test)]
mod tests {
    use syn::parse_str;

    use crate::parse::MicrotypeMacro;

    use super::*;

    #[test]
    fn removes_secret() {
        let microtype: MicrotypeMacro =
            parse_str("#[derive(Foo)] #[secret] #[string] String { Email }").unwrap();
        let attrs = microtype.0[0].attrs.clone();

        let (attrs, SpecialAttrs { secret, string }) = strip_special_attrs(attrs).unwrap();
        assert!(attrs.len() == 1);
        assert!(secret.is_some());
        assert!(string);
    }

    #[test]
    fn removes_secret_serialize() {
        let microtype: MicrotypeMacro =
            parse_str("#[derive(Foo)] #[secret(serialize)] String { Email }").unwrap();
        let attrs = microtype.0[0].attrs.clone();

        let (attrs, SpecialAttrs { secret, .. }) = strip_special_attrs(attrs).unwrap();
        assert!(attrs.len() == 1);
        assert!(secret.is_some());
        assert!(secret.unwrap().serialize.is_some());
    }
}