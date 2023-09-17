use proc_macro2::TokenStream;
use quote::{
    ToTokens, TokenStreamExt
};
use syn::{
    Result, Ident, Stmt,
    Item, Attribute, Block
};
use syn::token::{
    Brace, Mod
};
use syn::parse::{
    Parse, ParseStream
};

use super::{
    block_to_tokens,
    attribute_name_to_bytes
};

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

        for item in &mut target.content.unwrap().1 {
            if let Item::Fn(function) = item {
                let mut attributes = function.attrs.iter().filter_map(attribute_name_to_bytes);
                let mut skip: bool = false;

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

                if !skip {
                    contents.push(core::mem::replace(item, Item::Verbatim(TokenStream::new())));
                }
            }
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

        for item in &self.contents {
            match item {
                Item::Fn(function) => {
                    let is_test = function.attrs.iter().filter_map(attribute_name_to_bytes).any(|attr|{
                        attr == b"test"
                    });

                    if is_test {
                        suite.append_all(function.attrs.iter());
                        function.vis.to_tokens(&mut suite);
                        function.sig.to_tokens(&mut suite);

                        function.block.brace_token.surround(&mut suite, | inner | {
                            self.setup.to_tokens(inner);
                            inner.append_all(function.block.stmts.iter());
                            self.teardown.to_tokens(inner);
                        });
                    }
                },
                _ => {
                    item.to_tokens(&mut suite);
                }
            }
        }
        Mod::default().to_tokens(tokens);
        self.name.to_tokens(tokens);

        let braced: Brace = Brace::default();
        braced.surround(tokens, | inner | suite.to_tokens(inner));
    }
}