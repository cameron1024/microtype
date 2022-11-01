use syn::{parse::Parse, Attribute, Ident, Token, Type};

pub fn find_diesel_attr(attrs: &[Attribute]) -> Option<Type> {
    let attr = attrs.iter().find(|f| match f.path.get_ident() {
        Some(ident) => ident == "diesel",
        None => false,
    });

    attr.cloned().and_then(to_type)
}

fn to_type(attr: Attribute) -> Option<Type> {
    let Inner { ty, .. } = attr.parse_args().ok()?;
    Some(ty)
}

#[allow(unused, clippy::large_enum_variant)]
struct Inner {
    sql_type: Ident,
    eq: Token![=],
    ty: Type,
}

impl Parse for Inner {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let sql_type = input.parse()?;
        let eq = input.parse()?;
        let ty = input.parse()?;
        Ok(Inner { sql_type, eq, ty })
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::parse_str;

    use crate::parse::MicrotypeMacro;

    use super::*;

    #[test]
    fn strip_diesel_type_test() {
        let MicrotypeMacro(vec) = parse_str("#[derive(Foo)] #[secret] #[diesel(sql_type = diesel::sql_type::Text)] String { Email }").unwrap();
        let attrs = vec[0].attrs.clone();
        let ty = find_diesel_attr(&attrs);
        let path = match ty.unwrap() {
            Type::Path(path) => path,
            _ => panic!(),
        };
        assert_eq!(
            path.to_token_stream().to_string(),
            "diesel :: sql_type :: Text"
        )
    }
}
