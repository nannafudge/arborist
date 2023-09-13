use quote::{quote, format_ident, ToTokens};
use proc_macro2::{TokenStream, Delimiter};

use syn::{
    Type, Ident,
    Result, Token,
    parse::{ParseStream, Parse}
};

mod mocks;
pub use mocks::get_mock;

#[derive(Clone)]
pub(crate) struct TestIdent {
    name: Ident,
    subtest: Ident
}

impl ToTokens for TestIdent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        format_ident!("{}_{}", self.name, self.subtest).to_tokens(tokens)
    }
}

impl Parse for TestIdent {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![.]>()?;
        let subtest: Ident = input.parse()?;

        Ok(Self {
            name: name,
            subtest: subtest
        })
    }
}

#[derive(Clone)]
pub(crate) struct ImplTest {
    pub test_ident: TestIdent,
    pub target: Type,
    pub function: Ident,
    pub test_body: Ident,
    pub test_args: Option<TokenStream>,
    pub test_setup: Option<TokenStream>,
}

impl Parse for ImplTest {
    fn parse(input: ParseStream) -> Result<Self> {
        let test_ident: TestIdent = input.parse()?;

        input.parse::<Token![for]>()?;
        let target: Type = input.parse()?;
        input.parse::<Token![.]>()?;
        let function: Ident = input.parse()?;
        input.parse::<Token![;]>()?;

        input.parse::<Token![use]>()?;
        let test_body: Ident = input.parse()?;
        let test_args: Option<TokenStream> = input.step(| cursor | {
            if let Some((content, _, next)) = cursor.group(Delimiter::Parenthesis) {
                return Ok((content.token_stream(), next));
            }

            Err(cursor.error("Expected brackets - ()"))
        }).ok();
        let _ = input.parse::<Token![;]>();

        // Additional (optional) test setup boilerplate config
        let _ = input.parse::<Ident>();
        let _ = input.parse::<Token![=]>();

        let test_setup: Option<TokenStream> = input.step(| cursor | {
            if let Some((content, _, next)) = cursor.group(Delimiter::Brace) {
                return Ok((content.token_stream(), next));
            }

            Err(cursor.error("Expected braces - {}"))
        }).ok();

        Ok(Self {
            test_ident,
            target,
            function,
            test_body,
            test_args,
            test_setup
        })
    }
}

pub(crate) fn render_impl_test(parsed: ImplTest) -> proc_macro::TokenStream {
    let test_ident: TokenStream = parsed.test_ident.to_token_stream();
    let subtest = parsed.test_ident.subtest;

    let target = parsed.target;
    let function = parsed.function;
    let test_body = parsed.test_body;
    let test_args = parsed.test_args;
    let test_setup = parsed.test_setup;

    let expanded: proc_macro2::TokenStream = quote! {
        #[test]
        fn #test_ident() {
            #test_setup

            #test_body!(#subtest(#target, #function args = #test_args));
        }
    };

    proc_macro::TokenStream::from(expanded)
}

pub(crate) fn render_impl_mock(name: Ident) -> proc_macro::TokenStream {
    get_mock(name).into()
}