use syn::{Attribute, Ident, Type, Visibility};

use crate::parse::MicrotypeMacro;

pub struct Microtype {
    pub inner: Type,
    pub name: Ident,
    pub vis: Visibility,
    pub attrs: Vec<Attribute>,
}

pub fn flatten(microtype_macro: MicrotypeMacro) -> Vec<Microtype> {
    let mut result = vec![];

    for decl in microtype_macro.0 {
        for attr_ident in decl.idents {
            let mut attrs = attr_ident.attributes;
            attrs.extend(decl.attrs.clone());
            let microtype = Microtype {
                attrs,
                inner: decl.inner.clone(),
                name: attr_ident.ident,
                vis: decl.vis.clone(),
            };

            result.push(microtype);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::parse_str;

    use super::*;

    #[test]
    fn correctly_flattens_microtypes() {
        let microtype_macro: MicrotypeMacro =
            parse_str("#[foo] #[secret] String { #[bar] Email, #[baz] Username }").unwrap();
        let microtypes = flatten(microtype_macro);
        let first = &microtypes[0];
        let second = &microtypes[1];

        assert_eq!(first.attrs.len(), 2);
        assert_eq!(first.attrs[1].to_token_stream().to_string(), "# [foo]");
        assert_eq!(first.attrs[0].to_token_stream().to_string(), "# [bar]");
        assert_eq!(first.inner.to_token_stream().to_string(), "String");
        assert_eq!(first.name.to_string(), "Email");

        assert_eq!(second.attrs.len(), 2);
        assert_eq!(second.attrs[1].to_token_stream().to_string(), "# [foo]");
        assert_eq!(second.attrs[0].to_token_stream().to_string(), "# [baz]");
        assert_eq!(second.inner.to_token_stream().to_string(), "String");
        assert_eq!(second.name.to_string(), "Username");
    }
}
