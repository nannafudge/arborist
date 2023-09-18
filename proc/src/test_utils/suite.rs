use proc_macro2::TokenStream;
use quote::{
    ToTokens, TokenStreamExt
};
use syn::{
    Result, Ident, Item, Attribute, ExprGroup, ItemFn
};
use syn::token::{
    Brace, Mod
};
use syn::parse::{
    Parse, ParseStream
};

use crate::common::{
    block_to_tokens,
    attribute_name_to_bytes,
    greedy_parse
};

use super::TestCase;

pub struct TestCaseGroup {
    tests: Vec<Item>
}

impl Parse for TestCaseGroup {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self{
            tests: greedy_parse(input, |_| Ok(()))?
        })
    }
}

#[derive(Clone)]
pub struct TestSuite {
    name: Ident,
    setup: Option<TokenStream>,
    teardown: Option<TokenStream>,
    contents: Vec<Item>
}

impl Parse for TestSuite {
    fn parse(input: ParseStream) -> Result<Self> {
        let target: syn::ItemMod = input.parse::<syn::ItemMod>()?;
        if target.content.is_none() {
            return Ok(Self{
                name: target.ident,
                setup: None,
                teardown: None,
                contents: Vec::new()
            });
        }

        let mut setup: Option<TokenStream> = None;
        let mut teardown: Option<TokenStream> = None;
        let mut contents: Vec<Item> = Vec::with_capacity(1);

        for item in &mut target.content.expect("NO CONTENT").1 {
            let mut skip: bool = false;

            if let Item::Fn(function) = item {
                let mut attributes = function.attrs.iter().filter_map(attribute_name_to_bytes);

                // These should be self-contained, similar to 'interpolation' functions
                // TODO: tidy up 'framework' approach according to above
                while let Some(attr) = attributes.next() {
                    match attr {
                        b"setup" => {
                            if setup.is_some() {
                                return Err(input.error("Duplicate test setup decl"));
                            }

                            setup = Some(block_to_tokens(&function.block));
                            skip = true;
                        },
                        b"teardown" => {
                            if teardown.is_some() {
                                return Err(input.error("Duplicate test teardown decl"));
                            }

                            teardown = Some(block_to_tokens(&function.block));
                            skip = true;
                        },
                        _ => {}
                    }
                }

                if skip {
                    continue;
                }
            }

            contents.push(core::mem::replace(item, Item::Verbatim(TokenStream::new())));
        }

        Ok(Self {
            name: target.ident,
            setup,
            teardown,
            contents
        })
    }
}

impl ToTokens for TestSuite {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut suite: TokenStream = TokenStream::new();
        let mut contents = self.contents.iter().peekable();

        while let Some(item) = contents.next() {
            if let Item::Fn(function) = item {
                let is_test = function.attrs.iter()
                    .filter_map(attribute_name_to_bytes)
                    .any(| attr | {
                        attr == b"test" || attr == b"test_case"
                    });

                if is_test {
                    encapsulate_with_suite(self, function, &mut suite);
                    continue;
                }
            }

            item.to_tokens(&mut suite);
        }

        Mod::default().to_tokens(tokens);
        self.name.to_tokens(tokens);

        let braced: Brace = Brace::default();
        braced.surround(tokens, | inner | suite.to_tokens(inner));
    }
}

fn encapsulate_with_suite(suite: &TestSuite, target: &ItemFn, tokens: &mut TokenStream) {
    tokens.append_all(target.attrs.iter());
    target.vis.to_tokens(tokens);
    target.sig.to_tokens(tokens);

    target.block.brace_token.surround(tokens, | inner | {
        suite.setup.to_tokens(inner);
        inner.append_all(target.block.stmts.iter());
        suite.teardown.to_tokens(inner);
    });
}