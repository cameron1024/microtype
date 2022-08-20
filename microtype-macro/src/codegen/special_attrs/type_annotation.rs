use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use syn::{spanned::Spanned, Attribute};

fn duplicate_string(span: Span) -> TokenStream {
    quote_spanned!(span => compile_error!("duplicate `string` attribute found"))
}

fn duplicate_int(span: Span) -> TokenStream {
    quote_spanned!(span => compile_error!("duplicate `int` attribute found"))
}

fn multiple_special_attrs() -> TokenStream {
    quote::quote! { compile_error!("only one of `#[int]`, `#[string]` allowed") }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeAnnotation {
    String,
    Int,
}

pub fn strip_type_annotation(
    attrs: Vec<Attribute>,
) -> Result<(Vec<Attribute>, Option<TypeAnnotation>), TokenStream> {
    let (string, attrs): (Vec<_>, Vec<_>) = attrs
        .into_iter()
        .partition(|attr| attr.path.is_ident("string"));

    let string = match &string[..] {
        [] => false,
        [_single] => true,
        [_, second, ..] => return Err(duplicate_string(second.span())),
    };

    let (int, attrs): (Vec<_>, Vec<_>) = attrs
        .into_iter()
        .partition(|attr| attr.path.is_ident("int"));

    let int = match &int[..] {
        [] => false,
        [_single] => true,
        [_, second, ..] => return Err(duplicate_int(second.span())),
    };

    let type_annotations = match (string, int) {
        (false, false) => None,
        (true, false) => Some(TypeAnnotation::String),
        (false, true) => Some(TypeAnnotation::Int),
        _ => return Err(multiple_special_attrs()),
    };

    Ok((attrs, type_annotations))
}

#[cfg(test)]
mod tests {
    use syn::parse_str;

    use crate::parse::MicrotypeMacro;

    use super::*;

    #[test]
    fn strips_type_attr() {
        let microtype: MicrotypeMacro =
            parse_str("#[derive(Foo)] #[string] String { Email }").unwrap();
        let attrs = microtype.0[0].attrs.clone();

        let (attrs, type_annotations) = strip_type_annotation(attrs).unwrap();

        assert_eq!(attrs.len(), 1);
        assert_eq!(type_annotations, Some(TypeAnnotation::String));
    }

    #[test]
    fn fails_if_int_and_string() {
        let microtype: MicrotypeMacro = parse_str("#[int] #[string] String { Email }").unwrap();
        let attrs = microtype.0[0].attrs.clone();
        strip_type_annotation(attrs).unwrap_err();
    }
}
