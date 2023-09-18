use proc_macro2::TokenStream;
use std::collections::BTreeMap;

use crate::common::{
    greedy_parse_with,
    render_let_stmt
};

use quote::{
    ToTokens, TokenStreamExt,
    format_ident
};

use syn::{
    Ident, Expr,
    Type, FnArg,
    Item, ItemFn,
    Result, Token,
    parse::{
        Parse, ParseStream
    }
};

use super::{
    Print, Mutate,
    KeyValue
};


#[derive(Clone)]
pub(crate) struct ArgCreateFor(Vec<Ident>);

impl Parse for ArgCreateFor {
    fn parse(input: ParseStream) -> Result<Self> {
        let items: Vec<Ident> = greedy_parse_with(input, | input_after: ParseStream | {
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
pub(crate) struct ArgWith(Vec<KeyValue<Ident, Expr>>);

impl Parse for ArgWith {
    fn parse(input: ParseStream) -> Result<Self> {
        let items: Vec<KeyValue<Ident, Expr>> = greedy_parse_with(input, | input_after: ParseStream | {
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