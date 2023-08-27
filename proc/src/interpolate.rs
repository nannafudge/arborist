use std::collections::BTreeMap;

use syn::{Expr, Result, Token};
use syn::parse::{ParseStream, ParseBuffer, Parse};
use proc_macro2::{Ident, Delimiter, TokenStream};

pub(crate) struct InterpolateExpr {
    pub template: Expr,
    pub vals: BTreeMap<String, TokenStream>
}

fn parse_kv<'a>(input: &ParseBuffer<'a>) -> Result<(Ident, TokenStream)> {
    let name: Ident = input.parse()?;
    input.parse::<Token![=>]>()?;
    input.step(| cursor | {
        if let Some((content, _, next)) = cursor.group(Delimiter::Brace) {
            return Ok(((name, content.token_stream()), next));
        }

        Err(cursor.error("Interpolation values must be in braces - {}"))
    })
}

impl Parse for InterpolateExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let template = input.parse()?;
        input.parse::<Token![,]>()?;
        
        let mut values: BTreeMap<String, TokenStream> = BTreeMap::new();
        while let Ok((name, content)) = parse_kv(input) {
            values.insert(name.to_string(), content);
        }

        Ok(Self {
            template: template,
            vals: values
        })
    }
}