use quote::ToTokens;
use proc_macro2::TokenStream;
use crate::common::attribute_name_to_bytes;

use syn::{
    Result,
    Item, Ident,
    token::{
        Brace, Mod
    },
    parse::{
        Parse, ParseStream
    }
};

use super::{
    Mutate, Mutators,
    InsertUnique,
    macros::{
        impl_arg_errors,
        impl_unique_arg
    }
};

mod args;
use args::*;

#[repr(u8)]
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
enum SuiteMutator {
    // Mutators should be defined in the order they must apply
    Setup(ArgSetup),
    Teardown(ArgTeardown)
}

impl_arg_errors!(
    SuiteMutator,
    SuiteMutator::Setup(_) => "#[setup] statement",
    SuiteMutator::Teardown(_) => "#[teardown] statement"
);

impl Mutate for SuiteMutator {
    fn mutate(&self, target: &mut Item) {
        match self {
            SuiteMutator::Setup(arg) => arg.mutate(target),
            SuiteMutator::Teardown(arg) => arg.mutate(target)
        };
    }
}

#[derive(Clone)]
pub struct TestSuite {
    name: Ident,
    mutators: Mutators<SuiteMutator>,
    contents: Vec<Item>
}

impl Parse for TestSuite {
    fn parse(input: ParseStream) -> Result<Self> {
        let target: syn::ItemMod = input.parse::<syn::ItemMod>()?;
        if target.content.is_none() {
            return Ok(Self{
                name: target.ident,
                mutators: Mutators::new(),
                contents: Vec::new()
            });
        }

        let mut mutators: Mutators<SuiteMutator> = Mutators::new();
        let mut contents: Vec<Item> = Vec::with_capacity(1);

        for item in &mut target.content.expect("NO CONTENT").1 {
            let mut is_suite_arg: bool = false;

            if let Item::Fn(function) = item {
                let mut attributes = function.attrs.iter();

                // TODO: Make suites composable using 'use', where setup/teardown
                // functions are combined into one as an inheritable strategy
                while let Some(attr) = attributes.next() {
                    match attribute_name_to_bytes(attr) {
                        Some(b"setup") => {
                            mutators.insert_unique(
                                SuiteMutator::Setup(
                                    ArgSetup(function.block.stmts.to_owned())
                                )
                            )?;

                            is_suite_arg = true;
                        },
                        Some(b"teardown") => {
                            mutators.insert_unique(
                                SuiteMutator::Teardown(
                                    ArgTeardown(function.block.stmts.to_owned())
                                )
                            )?;

                            is_suite_arg = true;
                        },
                        _ => {}
                    }
                }
            }

            if !is_suite_arg {
                contents.push(core::mem::replace(item, Item::Verbatim(TokenStream::new())));
            }

        }

        Ok(Self {
            name: target.ident,
            mutators,
            contents
        })
    }
}

pub fn render_test_suite(mut test_suite: TestSuite) -> TokenStream {
    let mut suite_out: TokenStream = TokenStream::new();
    let mut contents = test_suite.contents.iter_mut();
    let braced: Brace = Brace::default();

    Mod::default().to_tokens(&mut suite_out);
    test_suite.name.to_tokens(&mut suite_out);
    braced.surround(&mut suite_out, | suite_inner | {
        while let Some(item) = contents.next() {
            if let Item::Fn(function) = item {
                let is_test = function.attrs.iter()
                    .filter_map(attribute_name_to_bytes)
                    .any(| attr | {
                        attr == b"test" || attr == b"test_case"
                    });

                if is_test {
                    test_suite.mutators.mutate(item);
                }
            }

            item.to_tokens(suite_inner);
        }
    });

    suite_out
}