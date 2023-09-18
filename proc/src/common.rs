use quote::{TokenStreamExt, ToTokens};
use syn::parse::{
    ParseStream, Parse
};
use syn::{
    Attribute, Result,
    Block, Ident,
    Type
};
use proc_macro2::{
    TokenStream,
    Delimiter
};

pub fn extract_attribute_inner(input: ParseStream) -> syn::Result<TokenStream> {
    input.parse::<syn::Token![#]>()?;
    parse_delim(proc_macro2::Delimiter::Bracket, &input)
}

pub fn attribute_name_to_bytes<'c>(attr: &Attribute) -> Option<&'c [u8]> {
    let name: Option<&'c [u8]> = attr.meta.path().get_ident().map(| ident: &syn::Ident | {
        steal(ident.to_string().as_bytes())
    });

    name
}

pub fn block_to_tokens(body: &Block) -> TokenStream {
    let mut out: TokenStream = TokenStream::new();
    out.append_all(&body.stmts);

    out
}

pub fn parse_delim<'c>(delim: Delimiter, input: ParseStream<'c>) -> Result<TokenStream> {
    input.step(| cursor | {
        if let Some((content, _, next)) = cursor.group(delim) {
            return Ok((content.token_stream(), next));
        }

        Err(cursor.error(format!("Expected delimiter: {:?}", delim)))
    })
}

pub fn greedy_parse<T, F, O>(input: ParseStream, after_hook: F) -> Result<Vec<T>> where
    T: Parse,
    F: for<'a> Fn(ParseStream<'a>) -> Result<O>
{
    let mut out: Vec<T> = Vec::with_capacity(1);
    while !input.is_empty() {
        out.push(input.parse::<T>()?);
        after_hook(input)?;
    }

    Ok(out)
}

pub fn render_let_stmt<T: ToTokens>(ident: &Ident, ty: &Type, value: &T, tokens: &mut TokenStream) {
    syn::token::Let::default().to_tokens(tokens);
    ident.to_tokens(tokens);
    syn::token::Colon::default().to_tokens(tokens);
    ty.to_tokens(tokens);
    syn::token::Eq::default().to_tokens(tokens);
    value.to_tokens(tokens);
    syn::token::Semi::default().to_tokens(tokens);
}

#[inline]
pub fn steal<'c, T: ?Sized>(item: &T) -> &'c T {
    unsafe {
        core::mem::transmute::<&T, &'c T>(item)
    }
}