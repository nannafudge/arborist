use quote::{quote, format_ident, ToTokens};
use proc_macro2::{TokenStream, Delimiter, Literal};

use syn::{
    Result, Token,
    Ident, LitInt,
    parse::{
        ParseStream, Parse
    }
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
    pub ident: TestIdent,
    pub body: TokenStream,
    pub iterations: LitInt
}

impl Parse for ImplTest {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: TestIdent = input.parse()?;
        input.parse::<Token![;]>()?;

        let mut iterations: LitInt = LitInt::from(Literal::usize_unsuffixed(1));
        let mut body: TokenStream = TokenStream::new();

        // Additional (optional) test setup boilerplate config
        while let Ok(test_suite_args) = input.parse::<Ident>() {
            input.parse::<Token![=]>()?;
            match test_suite_args.to_string().as_str() {
                "body" => {
                    body = input.step(| cursor | {
                        if let Some((content, _, next)) = cursor.group(Delimiter::Brace) {
                            return Ok((content.token_stream(), next));
                        }

                        Err(cursor.error("Expected braces - {}"))
                    })?;
                },
                "iterations" => {
                    iterations = input.parse::<LitInt>()?;
                },
                _ => {
                    input.error("Invalid test suite arg");
                }
            }
            input.parse::<Token![;]>()?;
        }

        Ok(Self {
            ident,
            body,
            iterations
        })
    }
}

pub(crate) fn render_impl_test(parsed: ImplTest) -> proc_macro::TokenStream {
    let test_ident: TokenStream = parsed.ident.to_token_stream();

    // TODO: Move this into ToTokens impl for ImplTest
    let test_body = parsed.body;
    let iterations = parsed.iterations;

    let expanded: proc_macro2::TokenStream = quote! {
        #[test]
        fn #test_ident() {
            for _ in 0..#iterations {
                #test_body
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

pub(crate) fn render_impl_mock(name: Ident) -> proc_macro::TokenStream {
    get_mock(name).into()
}