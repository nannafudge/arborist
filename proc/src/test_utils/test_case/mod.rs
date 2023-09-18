use quote::ToTokens;

use crate::common::{
    attribute_name_to_bytes,
    parse_delim
};

use proc_macro2::{
    Delimiter, Span,
    TokenStream
};

use syn::{
    Result, Token,
    Ident, Item,
    ItemFn, Attribute,
    AttrStyle,
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
enum TestPrinter {
    // Printers should be defined in the order they must apply
    UNIMPLEMENTED
}

impl Print for TestPrinter {
    fn print(&self, _: &Item, _: &mut TokenStream) {
        unimplemented!("No printers currently defined for test_case!");
    }
}

impl_arg_errors!(TestPrinter, TestPrinter::UNIMPLEMENTED => "UNIMPLEMENTED");

#[repr(u8)]
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
enum TestMutator {
    // Mutators should be defined in the order they must apply
    ArgName(ArgName),
    ArgWith(ArgWith)
}

impl Mutate for TestMutator {
    fn mutate(&self, target: &mut Item) {
        match self {
            TestMutator::ArgWith(arg) => arg.mutate(target),
            TestMutator::ArgName(arg) => arg.mutate(target)
        };
    }
}

impl_arg_errors!(
    TestMutator,
    TestMutator::ArgWith(_) => "with(...) arg",
    TestMutator::ArgName(name) => &format!("test name: {}", name.0.to_string())
);

#[allow(dead_code)]
#[derive(Clone)]
enum TestArgs {
    Printer(TestPrinter),
    Mutator(TestMutator)
}

impl Parse for TestArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;
        match name.to_string().as_bytes() {
            b"with" => {
                let inner: TokenStream = parse_delim(Delimiter::Parenthesis, input)?;
                Ok(TestArgs::Mutator(
                    TestMutator::ArgWith(syn::parse2::<ArgWith>(inner)?)
                ))
            },
            _ => {
                // Assume the ident is the test name
                Ok(TestArgs::Mutator(
                    TestMutator::ArgName(ArgName(name))
                ))
            }
        }
    }
}

#[derive(Clone)]
pub struct TestCase {
    printers: Printers<TestPrinter>,
    mutators: Mutators<TestMutator>
}

impl Parse for TestCase {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut printers: Printers<TestPrinter> = Printers::new();
        let mut mutators: Mutators<TestMutator> = Mutators::new();
        
        while !input.is_empty() {
            match input.parse::<TestArgs>()? {
                TestArgs::Printer(printer) => {
                    printers.insert_unique(printer)?;
                },
                TestArgs::Mutator(mutator) => {
                    mutators.insert_unique(mutator)?;
                }
            }

            // If more args to be parsed
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            printers,
            mutators
        })
    }
}

pub fn render_test_case(test_case: TestCase, mut target: ItemFn) -> TokenStream {
    let mut out: TokenStream = TokenStream::new();
    let mut test_cases: Vec<TestCase> = vec![test_case];

    // Search for other test case attributes, plucking such from the fn def if present
    for i in 0..target.attrs.len() {
        if attribute_name_to_bytes(&target.attrs[i]) == Some(b"test_case") {
            test_cases.push(
                target.attrs.remove(i)
                    .parse_args_with(TestCase::parse)
                    .expect("Error parsing test case")
                );
        }
    }

    for test in test_cases {
        let mut test_case_out: TokenStream = TokenStream::new();

        let mut target_fn: Item = {
            let mut local_fn: ItemFn = target.clone();
            local_fn.attrs.push(test_attribute());
            Item::Fn(local_fn)
        };

        // Apply mutators first (ala. only transforms/mutates elements)
        test.mutators.iter().for_each(| m | m.mutate(&mut target_fn));

        // Apply printers (ala. only creates elements) last

        // If printers are present, allow such to control
        // the rendering of the function test function -
        // else, render the function
        if test.printers.is_empty() {
            target_fn.to_tokens(&mut test_case_out);
        } else {
            test.printers.iter().for_each(| p | p.print(&target_fn, &mut test_case_out));
        }

        test_case_out.to_tokens(&mut out);
    }

    out
}

fn test_attribute() -> Attribute {
    Attribute {
        pound_token: syn::token::Pound::default(),
        style: AttrStyle::Outer,
        bracket_token: syn::token::Bracket::default(),
        meta: syn::Meta::Path(syn::Path::from(Ident::new_raw("test", Span::call_site()))),
    }
}