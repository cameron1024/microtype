use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Ident, LitStr, Result, Token, Type, Visibility,
};

/// The `= "foo::Bar"` part of the diesel type attribute
pub struct DieselTypeAttr {
    pub ty: Ident,
}

impl Parse for DieselTypeAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let _: Token![=] = input.parse()?;
        let s: LitStr = input.parse()?;
        let ty = Ident::new(&s.value(), s.span());
        Ok(Self { ty })
    }
}

/// The entire invocation of the macro
pub struct MicrotypeMacro(pub Vec<MicrotypeDecl>);

impl Parse for MicrotypeMacro {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut result = vec![];
        while !input.is_empty() {
            result.push(input.parse()?);
        }
        Ok(Self(result))
    }
}

/// A one-to-many mapping of inner type to any number of microtypes
pub struct MicrotypeDecl {
    pub attrs: Vec<Attribute>,
    pub inner: Type,
    pub idents: Vec<AttrIdent>,
    pub vis: Visibility,
}

impl Parse for MicrotypeDecl {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse()?;
        let inner = input.parse()?;
        let content;
        let _ = braced!(content in input);
        let idents = Punctuated::<AttrIdent, Token![,]>::parse_terminated(&content)?;
        let idents = idents.into_iter().collect();

        Ok(Self {
            attrs,
            inner,
            idents,
            vis,
        })
    }
}

/// Identifier with 0 or more attributes
pub struct AttrIdent {
    pub attributes: Vec<Attribute>,
    pub ident: Ident,
}

impl Parse for AttrIdent {
    fn parse(input: ParseStream) -> Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let ident = input.parse()?;
        Ok(Self { attributes, ident })
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_str;

    use super::*;

    #[test]
    fn parse_attr_ident() {
        let attr_ident: AttrIdent = parse_str("#[foo] asdf").unwrap();
        assert_eq!(attr_ident.attributes.len(), 1);
        assert_eq!(attr_ident.ident.to_string(), "asdf");
    }

    #[test]
    fn parse_microtype_decl() {
        let microtype_decl: MicrotypeDecl =
            parse_str("#[secret(serialize)] String { #[foo] Email }").unwrap();
        assert!(microtype_decl.attrs.len() == 1);
        assert_eq!(microtype_decl.idents[0].attributes.len(), 1);
        assert_eq!(microtype_decl.idents[0].ident.to_string(), "Email");
    }

    #[test]
    fn parse_full_macro() {
        let microtype: MicrotypeMacro = parse_str(
            r#"
#[foo]
#[secret(serialize)]
String {
    Email
}
i64 {
    Age
}
"#,
        )
        .unwrap();

        assert_eq!(microtype.0.len(), 2);
        let first = &microtype.0[0];
        assert_eq!(first.attrs.len(), 2);
        let ty = &first.inner;
        let ty = quote::quote! {#ty};
        assert_eq!(ty.to_string(), "String");
    }
}
