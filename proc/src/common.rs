// Not all functions may be used presently, but may be useful later
#![allow(dead_code, unused_macros)]
use quote::{
    TokenStreamExt, ToTokens
};
use proc_macro2::{
    Delimiter,
    TokenStream, TokenTree
};
use syn::{
    Block, Generics,
    Attribute, Result,
    parse::{
        ParseStream, Parse
    }
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

pub fn greedy_parse<T>(input: ParseStream) -> Result<Vec<T>> where
    T: Parse
{
    let mut out: Vec<T> = Vec::with_capacity(1);
    while !input.is_empty() {
        out.push(input.parse::<T>()?);
    }

    Ok(out)
}

pub fn greedy_parse_with<T, F, O>(input: ParseStream, after_hook: F) -> Result<Vec<T>> where
    T: Parse,
    F: for<'a> Fn(ParseStream<'a>) -> Result<O>
{
    let mut out: Vec<T> = Vec::with_capacity(1);
    while !input.is_empty() {
        out.push(input.parse::<T>()?);
        if !input.is_empty() {
            after_hook(input)?;
        }
    }

    Ok(out)
}

pub fn peek_next_tt(input: ParseStream) -> Result<TokenTree> {
    input.step(| cursor | {
        if let Some((tt, _)) = cursor.token_tree() {
            return Ok((tt, *cursor));
        }

        Err(cursor.error("Unexpected end of stream: Expected tokens"))
    })
}

pub fn result_to_tokens<T: ToTokens>(res: Result<T>, out: &mut TokenStream) {
    match res {
        Ok(item) => item.to_tokens(out),
        Err(e) => e.to_compile_error().to_tokens(out)
    }
}

pub fn result_to_tokens_with<T, F: Fn(T, &mut TokenStream)>(res: Result<T>, out: &mut TokenStream, func: F) {
    match res {
        Ok(item) => func(item, out),
        Err(e) => e.to_compile_error().to_tokens(out)
    }
}

#[inline]
pub fn steal<'c, T: ?Sized>(item: &T) -> &'c T {
    unsafe {
        core::mem::transmute::<&T, &'c T>(item)
    }
}

pub fn extract_impl_generics(generics: Option<&Generics>) -> TokenStream {
    generics.and_then(| generics | {
        let (impl_generics, _, _) = generics.split_for_impl();
        Some(impl_generics.to_token_stream())
    }).unwrap_or_default()
}

pub fn extract_ty_and_where_generics(generics: Option<&Generics>) -> (TokenStream, TokenStream) {
    generics.and_then(| generics | {
        let (_, ty_generics, where_clause) = generics.split_for_impl();
        Some((ty_generics.to_token_stream(), where_clause.to_token_stream()))
    }).unwrap_or_default()
}

#[macro_use]
mod macros {
    macro_rules! error_spanned {
        ($formatter:literal, $item:expr $(, $other_items:expr )*) => {
            syn::Error::new(syn::spanned::Spanned::span($item), &format!(
                $formatter, quote::ToTokens::to_token_stream($item) $(, quote::ToTokens::to_token_stream($other_items))*
            ))
        };
    }
    
    pub(crate) use error_spanned;
}

// Compiler u very dumbb
#[allow(unused_imports)]
pub(crate) use macros::error_spanned;