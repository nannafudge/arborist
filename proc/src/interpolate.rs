use syn::{
    Result, LitStr,
    Token
};
use syn::parse::{
    ParseStream, ParseBuffer,
    Parse,
};
use proc_macro2::{
    Delimiter, Ident,
    TokenTree, TokenStream,
};
use std::collections::BTreeMap;
use quote::ToTokens;

#[inline]
fn to_token_stream(input: &str) -> syn::Result<TokenStream> {
    match syn::parse_str::<TokenStream>(&input) {
        Ok(ts) => Ok(ts),
        Err(e) => Err(syn::Error::new(e.span(), format!{"{}", e}))
    }
}

fn parse_kv(input: &ParseBuffer) -> Result<(Ident, TokenStream)> {
    let name: Ident = input.parse()?;
    input.parse::<Token![=>]>()?;

    input.step(| cursor | {
        if let Some((content, _, next)) = cursor.group(Delimiter::Brace) {
            let maybe_function: Result<Function> = syn::parse2(content.token_stream());
            if let Ok(function) = &maybe_function {
                return Ok(((name, execute_function(function)?), next));
            }

            return Ok(
                ((name, content.token_stream()), next)
            );
        }

        Err(cursor.error("Interpolation values must be in braces - {}"))
    })
}

fn parse_kvs(input: &ParseBuffer) -> BTreeMap<String, TokenStream> {
    let mut kvs: BTreeMap<String, TokenStream> = BTreeMap::new();
    while let Ok((key, content)) = parse_kv(input) {
        kvs.insert(key.to_string(), content);
    }
    kvs
}

fn execute_function(function: &Function) -> Result<TokenStream> {
    match function {
        Function::Format(function) => {
            let mut out: String = function.formatter.value();
            for (name, arg) in &function.args {
                out = out.replace(&format!("#[{}]", name), arg.to_string().as_str());
            }

            to_token_stream(&out)
        },
        Function::Select(function) => {
            let mut out: TokenStream = TokenStream::new();
            match function.selector {
                true => function.right.to_tokens(&mut out),
                false => function.left.to_tokens(&mut out)
            }

            Ok(out)
        }
    }
}

pub(crate) struct Interpolate {
    pub template: TokenStream,
    pub vals: BTreeMap<String, TokenStream>
}

enum Function {
    Format(FnFormat),
    Select(FnSelect)
}

struct FnFormat {
    formatter: LitStr,
    args: BTreeMap<String, TokenStream>
}

struct FnSelect {
    left: TokenStream,
    right: TokenStream,
    selector: bool
}

impl Parse for FnFormat {
    fn parse(input: ParseStream) -> Result<Self> {
        let formatter: LitStr = input.parse()?;

        Ok(
            Self{
                formatter: formatter,
                args: parse_kvs(input)
            }
        )
    }
}

impl Parse for FnSelect {
    fn parse(input: ParseStream) -> Result<Self> {
        let left = parse_kv(input)?.1;
        let right = parse_kv(input)?.1;
        let selector = parse_kv(input)?.1;

        Ok(Self {
            left: left,
            right: right,
            selector: selector.to_string().as_str() == ""
        })
    }
}

impl Parse for Function {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let args: TokenStream = input.step(| cursor | {
            if let Some((content, _, next)) = cursor.group(Delimiter::Parenthesis) {
                return Ok((content.token_stream(), next));
            }

            Err(cursor.error("Expected brackets - ()"))
        })?;

        match name.to_string().as_str() {
            "format" => {
                Ok(Function::Format(syn::parse2::<FnFormat>(args)?))
            },
            "select" => {
                Ok(Function::Select(syn::parse2::<FnSelect>(args)?))
            },
            _ => {
                return Err(syn::Error::new(
                    name.span(),
                    format!("Unrecognized interpolation function: {}", name.to_string())
                ));
            }
        }
    }
}

impl Parse for Interpolate {
    fn parse(input: ParseStream) -> Result<Self> {
        let values: BTreeMap<String, TokenStream> = parse_kvs(&input);
        input.parse::<Token![,]>()?;

        let mut template: TokenStream = TokenStream::new();
        while let Ok(token) = input.parse::<TokenTree>() {
            token.to_tokens(&mut template);
        }

        Ok(Self {
            template: template,
            vals: values
        })
    }
}