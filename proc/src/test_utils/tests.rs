use quote::{
    quote, format_ident,
    ToTokens, TokenStreamExt
};
use proc_macro2::{
    TokenStream, Delimiter,
    Literal
};
use syn::{
    Result, Token,
    Ident, Expr,
    ItemFn
};
use syn::parse::{
    Parse, ParseStream
};

#[derive(Clone)]
pub struct TestCase {
    with: Vec<(Ident, Expr)>
}

/*impl Parse for TestCase {
    fn parse(input: ParseStream) -> Result<Self> {
        let target: ItemFn = input.parse::<ItemFn>()?;
        target.attrs.
    }
}*/