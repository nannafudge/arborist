use crate::common::attribute_name_to_bytes;
use proc_macro2::TokenStream;
use quote::ToTokens;

use syn::{
    Item, Stmt,
    Result, Ident,
    token::{
        Brace, Mod
    },
    parse::{
        Parse, ParseStream
    }
};

use super::{
    InsertUnique,
    Print, Printers,
    Mutate, Mutators,
    macros::{
        impl_arg_errors,
        impl_unique_arg
    }
};

mod args;
use args::*;

#[repr(u8)]
#[allow(dead_code)]
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
enum SuitePrinter {
    // Printers should be defined in the order they must apply
    UNIMPLEMENTED
}

impl_arg_errors!(SuitePrinter, SuitePrinter::UNIMPLEMENTED => "UNIMPLEMENTED");

impl Print for SuitePrinter {
    fn print(&self, _: &Item, _: &mut TokenStream) {
        unimplemented!("No printer implementations exist for TestSuites yet");
    }
}

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
    printers: Printers<SuitePrinter>,
    mutators: Mutators<SuiteMutator>,
    contents: Vec<Item>
}

impl Parse for TestSuite {
    fn parse(input: ParseStream) -> Result<Self> {
        let target: syn::ItemMod = input.parse::<syn::ItemMod>()?;
        if target.content.is_none() {
            return Ok(Self{
                name: target.ident,
                printers: Printers::new(),
                mutators: Mutators::new(),
                contents: Vec::new()
            });
        }

        // WARN: This should be made mutable when a suite printer is implemented
        let printers: Printers<SuitePrinter> = Printers::new();
        let mut mutators: Mutators<SuiteMutator> = Mutators::new();
        let mut contents: Vec<Item> = Vec::with_capacity(1);

        for item in &mut target.content.expect("NO CONTENT").1 {
            let mut is_suite_arg: bool = false;

            if let Item::Fn(function) = item {
                let mut attributes = function.attrs.iter();

                while let Some(attr) = attributes.next() {
                    match attribute_name_to_bytes(attr) {
                        Some(b"setup") => {
                            // TODO: Make suites composable using 'use', where setup/teardown
                            // functions are combined into one as an inheritable strategy
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
            printers,
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
                    test_suite.mutators.iter().for_each(| m | m.mutate(item));

                    // Allow printers to control formatting/output of item
                    if !test_suite.printers.is_empty() {
                        test_suite.printers.iter().for_each(| p | p.print(item, suite_inner));
                        continue;
                    }
                }
            }
    
            item.to_tokens(suite_inner);
        }
    });

    suite_out
}