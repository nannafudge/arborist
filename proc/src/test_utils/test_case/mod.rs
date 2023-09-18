use crate::common::parse_delim;
use quote::ToTokens;

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
    KeyValue,
    Print, Mutate,
    Printers, Mutators,
    InsertUnique,
    macros::{
        impl_enum_debug,
        impl_arg_ord_traits
    }
};

mod args;
use args::*;

#[repr(u8)]
#[derive(Clone)]
enum TestPrinter {
    ArgCreateFor(ArgCreateFor)
}

impl Print for TestPrinter {
    fn print(&self, target: &Item, tokens: &mut TokenStream) {
        match self {
            TestPrinter::ArgCreateFor(arg) => arg.print(target, tokens)
        };
    }
}

impl_arg_ord_traits!(TestPrinter);
impl_enum_debug!(TestPrinter, TestPrinter::ArgCreateFor(_));

#[repr(u8)]
#[derive(Clone)]
enum TestMutator {
    ArgWith(ArgWith)
}

impl Mutate for TestMutator {
    fn mutate(&self, target: &mut Item) {
        match self {
            TestMutator::ArgWith(arg) => arg.mutate(target)
        };
    }
}

impl_arg_ord_traits!(TestMutator);
impl_enum_debug!(TestMutator, TestMutator::ArgWith(_));

#[derive(Clone)]
enum TestArgs {
    Printer(TestPrinter),
    Mutator(TestMutator)
}

impl Parse for TestArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?.to_string();
        match name.as_bytes() {
            b"create_for" => {
                let inner: TokenStream = parse_delim(Delimiter::Parenthesis, input)?;
                Ok(TestArgs::Printer(
                    TestPrinter::ArgCreateFor(syn::parse2::<ArgCreateFor>(inner)?)
                ))
            },
            b"with" => {
                let inner: TokenStream = parse_delim(Delimiter::Parenthesis, input)?;
                Ok(TestArgs::Mutator(
                    TestMutator::ArgWith(syn::parse2::<ArgWith>(inner)?)
                ))
            }
            _ => Err(input.error(format!("Unrecognized test case arg: {:?}", name)))
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

pub fn render_test_case(test_opts: TestCase, mut target: ItemFn) -> TokenStream {
    let mut out: TokenStream = TokenStream::new();

    let mut target_fn: Item = {
        target.attrs.push(test_attribute());
        Item::Fn(target)
    };

    // Apply mutators first (ala. only transforms/mutates elements)
    test_opts.mutators.iter().for_each(| m | m.mutate(&mut target_fn));

    // If printers are present, allow such to control
    // the rendering of the function test function -
    // else, render the function
    if test_opts.printers.is_empty() {
        target_fn.to_tokens(&mut out);
        return out;
    }

    // Apply printers (ala. only creates elements) last
    test_opts.printers.iter().for_each(| p | p.print(&target_fn, &mut out));

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