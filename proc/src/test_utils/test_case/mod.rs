use quote::ToTokens;
use crate::common::{
    peek_next_tt, parse_delim,
    attribute_name_to_bytes
};
use proc_macro2::{
    TokenStream,
    Delimiter, Span,
};
use syn::{
    Ident, Item, ItemFn, Token,
    Result, Attribute, AttrStyle,
    parse::{
        Parse, ParseStream
    },
    spanned::Spanned
};

use super::{
    InsertUnique,
    Mutate, Mutators,
    macros::*
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
    fn mutate(self, target: &mut Item) {
        match self {
            TestMutator::ArgWith(arg) => arg.mutate(target),
            TestMutator::ArgName(arg) => arg.mutate(target)
        };
    }
}

impl ToTokens for TestMutator {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TestMutator::ArgWith(arg) => arg.to_tokens(tokens),
            TestMutator::ArgName(arg) => arg.to_tokens(tokens)
        };
    }
}

impl Parse for TestMutator {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>().map_err(|_| {
            // If we've gotten to this point, next el should never be empty,
            // as TestCase.parse() iterates while !input.empty()
            let next = peek_next_tt(input).unwrap();
            input.error(format!("Unexpected argument: {}", next))
        })?;

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
pub struct TestCase(Mutators<TestMutator>);

impl Mutate for TestCase {
    fn mutate(mut self, target: &mut Item) {
        while let Some(mutator) = self.0.pop_first() {
            mutator.mutate(target);
        }
    }
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

        Ok(Self(mutators))
    }
}

impl_to_tokens_wrapped!(TestCase: collection);

fn parse_arg_parameterized<T: Parse>(input: ParseStream) -> Result<T> {
    let arg_inner: TokenStream = parse_delim(Delimiter::Parenthesis, input)?;
    syn::parse2::<T>(arg_inner)
}

pub fn render_test_case(test_case_: TestCase, mut target: ItemFn) -> TokenStream {
    let mut out: TokenStream = TokenStream::new();
    let mut test_cases: Vec<TestCase> = vec![test_case_];

    // Search for other test case attributes, plucking such from the fn def if present
    let mut removed_elements: usize = 0;
    for i in 0..target.attrs.len() {

        if attribute_name_to_bytes(&target.attrs[i - removed_elements]) == Some(b"test_case") {
            let attr = target.attrs.remove(i - removed_elements);
            let parsed_attr = attr.parse_args_with(TestCase::parse);
            if parsed_attr.is_err() {
                unsafe {
                    return parsed_attr.unwrap_err_unchecked().into_compile_error();
                }
            }

            test_cases.push(parsed_attr.unwrap());

            // Upon removal, the vec shifts one to
            // the left (and thus - so does the length)
            // So we must adjust index `i` accordingly
            removed_elements += 1;
        }
    }

    for test_case in test_cases {
        let mut target_fn: Item = {
            let mut local_fn: ItemFn = target.clone();
            local_fn.attrs.push(test_attribute(target.span()));
            Item::Fn(local_fn)
        };

        test_case.mutate(&mut target_fn);
        target_fn.to_tokens(&mut out);
    }

    out
}

fn test_attribute(span: Span) -> Attribute {
    Attribute {
        pound_token: syn::token::Pound::default(),
        style: AttrStyle::Outer,
        bracket_token: syn::token::Bracket::default(),
        meta: syn::Meta::Path(syn::Path::from(Ident::new_raw("test", span))),
    }
}