use syn::{
    Result,
    LitStr,
    parse::{Parse, ParseStream},
};

pub struct ImageAttr {
    pub path: LitStr,
}

impl Parse for ImageAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            path: input.parse()?,
        })
    }
}

#[cfg(test)]
mod test {
    use syn::parse_str;
    use super::*;

    #[test]
    fn path() {
        let params: ImageAttr = parse_str(r#""path/to/icon.png""#).unwrap();

        assert_eq!(params.path.value(), "path/to/icon.png");
    }
}
