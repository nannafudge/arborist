use super::{
    Print, Mutate
};
use crate::common::{
    greedy_parse, parse_delim,
    render_let_stmt, steal
};
use quote::{
    ToTokens, TokenStreamExt,
    format_ident
};
use proc_macro2::{
    Delimiter, Span,
    TokenStream
};
use syn::{
    Result, Token,
    Ident, Expr, Item,
    ItemFn, Attribute,
    AttrStyle, FnArg,
    Type
};
use syn::parse::{
    Parse, ParseStream
};

use std::collections::{
    BTreeSet,
    BTreeMap
};

#[derive(Clone)]
struct ArgCreateFor(Vec<Ident>);

impl Parse for ArgCreateFor {
    fn parse(input: ParseStream) -> Result<Self> {
        let items: Vec<Ident> = greedy_parse(input, | input_after: ParseStream | {
            if !input_after.is_empty() {
                input_after.parse::<Token![,]>()?;
            }

            Ok(())
        })?;

        Ok(Self(items))
    }
}

impl Print for ArgCreateFor {
    fn print(&self, target: &Item, tokens: &mut TokenStream) {
        if let Item::Fn(function) = target {
            for ident in &self.0 {
                let mut new_sig = function.sig.clone();
                new_sig.ident = format_ident!("{}_{}", new_sig.ident, ident.to_string().to_lowercase());

                tokens.append_all(function.attrs.iter());
                function.vis.to_tokens(tokens);
                new_sig.to_tokens(tokens);
                function.block.to_tokens(tokens);
            }

            return;
        }

        panic!("Invalid target for ArgCreateFor: {:?}, expected ItemFn", target.to_token_stream());
    }
}

#[derive(Clone)]
struct ArgWith(Vec<KeyValue<Ident, Expr>>);

impl Parse for ArgWith {
    fn parse(input: ParseStream) -> Result<Self> {
        let items: Vec<KeyValue<Ident, Expr>> = greedy_parse(input, | input_after: ParseStream | {
            if !input_after.is_empty() {
                input_after.parse::<Token![,]>()?;
            }

            Ok(())
        })?;

        Ok(Self(items))
    }
}

impl Mutate for ArgWith {
    fn mutate(&self, target: &mut Item) {
        if let Item::Fn(function) = target {
            let mut new_fn_def: TokenStream = TokenStream::new();

            // Append existing attributes and visibility modifiers
            new_fn_def.append_all(function.attrs.iter());
            function.vis.to_tokens(&mut new_fn_def);

            // Steal inputs from signature, leaving the original function sig inputs empty
            let inputs = core::mem::take(&mut function.sig.inputs);
            function.sig.to_tokens(&mut new_fn_def);

            let input_map: BTreeMap<String, Box<Type>> = BTreeMap::from_iter(
                inputs.iter().map(| arg | {
                    if let FnArg::Typed(inner) = arg {
                        if let syn::Pat::Ident(name) = inner.pat.as_ref() {
                            return (
                                name.ident.to_string(),
                                inner.ty.to_owned()
                            );
                        }
                    }
    
                    panic!("Unexpected test input: {:?}", arg.to_token_stream());
                })
            );

            // Insert input value statements parsed from attribute directly into fn body
            function.block.brace_token.surround(&mut new_fn_def, | test_body | {
                for stmt in &self.0 {
                    // Fetch the corresponding type from the previous function inputs
                    let stmt_ty: &Box<Type> = input_map.get(&stmt.k.to_string()).expect(
                        &format!("No corresponding input argument defined on test function signature for {:?}", stmt.k)
                    );

                    render_let_stmt(&stmt.k, stmt_ty, &stmt.v, test_body);
                }

                test_body.append_all(function.block.stmts.iter());
            });

            let new_fn: ItemFn = syn::parse2::<ItemFn>(new_fn_def)
                .expect("mutate(): Error creating new function def");

            // Replace the old function with the new mutation
            *target = Item::Fn(new_fn);

            return;
        }

        panic!("Invalid target for ArgCreateFor: {:?}, expected ItemFn", target.to_token_stream());
    }
}

#[derive(Clone)]
struct KeyValue<K: Parse, V: Parse> {
    k: K,
    v: V
}

impl<K, V> Parse for KeyValue<K, V> where
    K: Parse, V: Parse
{
    fn parse(input: ParseStream) -> Result<Self> {
        let key: K = input.parse::<K>()?;
        input.parse::<Token![=]>()?;
        
        Ok(Self {
            k: key,
            v: input.parse::<V>()?
        })
    }
}

#[repr(u8)]
#[derive(Clone)]
enum TestPrinter {
    ArgCreateFor(ArgCreateFor)
}

impl PartialEq for TestPrinter {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for TestPrinter {}

impl PartialOrd for TestPrinter {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        ((self as *const TestPrinter) as u8).partial_cmp(
            &((other as *const TestPrinter) as u8)
        )
    }
}

impl Ord for TestPrinter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("TestPrinter: Unexpected ord result")
    }
}

impl Print for TestPrinter {
    fn print(&self, target: &Item, tokens: &mut TokenStream) {
        match self {
            TestPrinter::ArgCreateFor(arg) => arg.print(target, tokens)
        };
    }
}

#[repr(u8)]
#[derive(Clone)]
enum TestMutator {
    ArgWith(ArgWith)
}

impl PartialEq for TestMutator {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for TestMutator {}

impl PartialOrd for TestMutator {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        ((self as *const TestMutator) as u8).partial_cmp(
            &((other as *const TestMutator) as u8)
        )
    }
}

impl Ord for TestMutator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("TestMutator: Unexpected ord result")
    }
}

impl Mutate for TestMutator {
    fn mutate(&self, target: &mut Item) {
        match self {
            TestMutator::ArgWith(arg) => arg.mutate(target)
        };
    }
}

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
    printers: BTreeSet<TestPrinter>,
    mutators: BTreeSet<TestMutator>
}

impl Parse for TestCase {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut printers: BTreeSet<TestPrinter> = BTreeSet::new();
        let mut mutators: BTreeSet<TestMutator> = BTreeSet::new();

        while !input.is_empty() {
            match input.parse::<TestArgs>()? {
                TestArgs::Printer(printer) => {
                    printers.insert(printer);
                },
                TestArgs::Mutator(mutator) => {
                    mutators.insert(mutator);
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

fn test_attribute() -> Attribute {
    Attribute {
        pound_token: syn::token::Pound::default(),
        style: AttrStyle::Outer,
        bracket_token: syn::token::Bracket::default(),
        meta: syn::Meta::Path(syn::Path::from(Ident::new_raw("test", Span::call_site()))),
    }
}

pub fn render_test_case(test_opts: TestCase, target: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut out: TokenStream = TokenStream::new();

    let mut target_fn: Item = {
        let mut inner: ItemFn = syn::parse_macro_input!(target as ItemFn);
        inner.attrs.push(test_attribute());

        Item::Fn(inner)
    };

    // Apply mutators first (ala. only transforms/mutates elements)
    for mutator in test_opts.mutators {
        mutator.mutate(&mut target_fn);
    }

    if test_opts.printers.len() == 0 {
        target_fn.to_tokens(&mut out);
        return out.into();
    }

    // If printers are present, allow such to control
    // the rendering of the function test function

    // Apply printers (ala. only creates elements) last
    for printer in test_opts.printers {
        printer.print(&target_fn, &mut out);
    }

    out.into()
}