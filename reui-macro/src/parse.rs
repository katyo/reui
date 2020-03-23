use syn::{
    Attribute,
    Visibility,
    token::{Static, Mut, Colon, Eq, Semi},
    Ident,
    Result,
    parse::{Parse, ParseStream},
};

pub struct StaticItem<Type, Value> {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub static_token: Static,
    pub mutability: Option<Mut>,
    pub ident: Ident,
    pub colon_token: Colon,
    pub ty: Type,
    pub eq_token: Eq,
    pub value: Value,
    pub semi_token: Semi,
}

impl<Type, Value> Parse for StaticItem<Type, Value>
where
    Type: Parse,
    Value: Parse,
{
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            vis: input.parse()?,
            static_token: input.parse()?,
            mutability: input.parse()?,
            ident: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

#[cfg(test)]
mod test {
    use syn::{parse_str, Type, LitChar, token::Paren, parenthesized};
    use super::*;

    struct Empty {
        _paren_token: Paren,
    }

    impl Parse for Empty {
        fn parse(input: ParseStream) -> Result<Self> {
            let _content;
            Ok(Empty {
                _paren_token: parenthesized!(_content in input),
            })
        }
    }

    #[test]
    fn no_value() {
        assert!(parse_str::<StaticItem<Type, Empty>>(r#"static IDENT: Type"#).is_err());
    }

    #[test]
    fn empty_value() {
        let item: StaticItem<Type, Empty> = parse_str(r#"static IDENT: Type = ();"#).unwrap();

        assert_eq!(item.ident.to_string(), "IDENT");
    }

    #[test]
    fn literal_value() {
        let item: StaticItem<Type, LitChar> = parse_str(r#"pub static IDENT: Type = 'a';"#).unwrap();

        assert_eq!(item.ident.to_string(), "IDENT");
        assert_eq!(item.value.value(), 'a');
    }
}
