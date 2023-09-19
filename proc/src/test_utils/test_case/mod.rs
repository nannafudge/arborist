use quote::ToTokens;
use crate::common::{
    parse_delim,
    attribute_name_to_bytes
};
use proc_macro2::{
    TokenStream,
    Delimiter, Span,
};
use syn::{
    Ident, Item,
    Result, Token,
    ItemFn, Attribute,
    AttrStyle,
    parse::{
        Parse, ParseStream
    }
};

use super::{
    InsertUnique,
    Mutate, Mutators,
    macros::{
        impl_arg_errors,
        impl_unique_arg
    }
};

mod args;
use args::*;

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

impl Parse for TestMutator {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;
        match name.to_string().as_bytes() {
            b"with" => {
                Ok(TestMutator::ArgWith(parse_arg_parameterized(input)?))
            },
            _ => {
                // Assume the ident is the test name
                Ok(TestMutator::ArgName(ArgName(name)))
            }
        }
    }
}

#[derive(Clone)]
pub struct TestCase {
    mutators: Mutators<TestMutator>
}

impl Parse for TestCase {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut mutators: Mutators<TestMutator> = Mutators::new();
        
        while !input.is_empty() {
            mutators.insert_unique(input.parse::<TestMutator>()?)?;

            // If more args to be parsed
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            mutators
        })
    }
}

fn parse_arg_parameterized<T: Parse>(input: ParseStream) -> Result<T> {
    let arg_inner: TokenStream = parse_delim(Delimiter::Parenthesis, input)?;
    syn::parse2::<T>(arg_inner)
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
        let mut target_fn: Item = {
            let mut local_fn: ItemFn = target.clone();
            local_fn.attrs.push(test_attribute());
            Item::Fn(local_fn)
        };

        test.mutators.mutate(&mut target_fn);
        target_fn.to_tokens(&mut out);
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