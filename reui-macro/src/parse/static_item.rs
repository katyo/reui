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

impl<Type, Value> StaticItem<Type, Value> {
    pub fn into<ToType, ToValue>(self) -> StaticItem<ToType, ToValue>
    where
        ToType: From<Type>,
        ToValue: From<Value>,
    {
        StaticItem {
            attrs: self.attrs,
            vis: self.vis,
            static_token: self.static_token,
            mutability: self.mutability,
            ident: self.ident,
            colon_token: self.colon_token,
            ty: self.ty.into(),
            eq_token: self.eq_token,
            value: self.value.into(),
            semi_token: self.semi_token,
        }
    }
}

pub trait StaticValueParse<Type>: Sized {
    fn parse_static_value(input: ParseStream, ident: &Ident, ty: &Type) -> Result<Self>;
}

impl<Type, Value> Parse for StaticItem<Type, Value>
where
    Type: Parse,
    Value: StaticValueParse<Type>,
{
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse()?;
        let static_token = input.parse()?;
        let mutability = input.parse()?;
        let ident = input.parse()?;
        let colon_token = input.parse()?;
        let ty = input.parse()?;
        let eq_token = input.parse()?;
        let value = Value::parse_static_value(input, &ident, &ty)?;
        let semi_token = input.parse()?;

        Ok(Self {
            attrs,
            vis,
            static_token,
            mutability,
            ident,
            colon_token,
            ty,
            eq_token,
            value,
            semi_token,
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

    impl StaticValueParse<Type> for Empty {
        fn parse_static_value(input: ParseStream, _ident: &Ident, _ty: &Type) -> Result<Self> {
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

    impl StaticValueParse<Type> for LitChar {
        fn parse_static_value(input: ParseStream, _ident: &Ident, _ty: &Type) -> Result<Self> {
            input.parse()
        }
    }

    #[test]
    fn literal_value() {
        let item: StaticItem<Type, LitChar> = parse_str(r#"pub static IDENT: Type = 'a';"#).unwrap();

        assert_eq!(item.ident.to_string(), "IDENT");
        assert_eq!(item.value.value(), 'a');
    }
}
