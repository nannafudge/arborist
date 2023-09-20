use proc_macro2::{
    TokenStream, TokenTree
};
use super::{
    Mutate,
    impl_unique_arg,
    impl_to_tokens_wrapped
};
use crate::common::{
    greedy_parse_with, error_spanned,
    result_to_tokens_with, result_to_tokens
};
use quote::{
    format_ident,
    ToTokens, TokenStreamExt,
};
use syn::{
    Ident, Expr, Item, Token,
    FnArg, ItemFn, Result, Stmt,
    parse::{
        Parse, ParseStream
    }
};

#[derive(Clone)]
pub struct ArgName(pub Ident);

impl Parse for ArgName {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(input.parse::<Ident>()?))
    }
}

impl Mutate for ArgName {
    fn mutate(self, target: &mut Item) {
        if let Item::Fn(function) = target {
            function.sig.ident = format_ident!("{}_{}", function.sig.ident, self.0);

            return;
        }
        
        panic!("{}", error_spanned!("#[test_case]: {} is not a function", target));
    }
}

impl_unique_arg!(ArgName);
impl_to_tokens_wrapped!(ArgName);

#[derive(Clone)]
pub(crate) struct ArgWith(Vec<Expr>);

impl Parse for ArgWith {
    fn parse(input: ParseStream) -> Result<Self> {
        let items: Vec<Expr> = greedy_parse_with(input, | input_after: ParseStream | {
            if !input_after.is_empty() {
                input_after.parse::<Token![,]>()?;
            }

            Ok(())
        })?;

        Ok(Self(items))
    }
}

impl Mutate for ArgWith {
    fn mutate(self, target: &mut Item) {
        if let Item::Fn(function) = target {
            let mut new_fn_def: TokenStream = TokenStream::new();

            // Append existing attributes and visibility modifiers
            new_fn_def.append_all(function.attrs.iter());
            function.vis.to_tokens(&mut new_fn_def);

            // Steal inputs from signature, leaving the original function sig inputs empty
            let inputs = core::mem::take(&mut function.sig.inputs);
            function.sig.to_tokens(&mut new_fn_def);

            // Extract fn inputs, preserving their order, mapping such to their ident & type
            let mut input_map = inputs.iter().map(| arg | {
                if let FnArg::Typed(item) = arg {
                    if let syn::Pat::Ident(decl) = item.pat.as_ref() {
                        return Ok(decl);
                    }

                    return Err(error_spanned!("{}: expected `ident: ty` mapping", item));
                }

                Err(error_spanned!("{}: Invalid input", arg))
            });

            // TODO: Switch this the other way around (go through fn args
            // matching such against with()) - it's more intuitive
            // Insert input value statements parsed from attribute directly into fn body
            function.block.brace_token.surround(&mut new_fn_def, | test_body | {
                for stmt in &self.0 {
                    let maybe_stmt_meta: Result<Result<&syn::PatIdent>> = input_map.next().ok_or(
                        error_spanned!("{}: missing corresponding argument", stmt)
                    );

                    result_to_tokens_with(maybe_stmt_meta, test_body, | stmt_meta, tokens | {
                        syn::token::Let::default().to_tokens(tokens);
                        result_to_tokens(stmt_meta, tokens);
                        syn::token::Eq::default().to_tokens(tokens);
                        stmt.to_tokens(tokens);
                        syn::token::Semi::default().to_tokens(tokens);
                    });
                }

                test_body.append_all(function.block.stmts.iter());
            });

            let new_fn = syn::parse2::<ItemFn>(new_fn_def);
            if new_fn.is_err() {
                panic!("{}", error_spanned!("with(): Error creating new function def: {}", &target))
            }

            // Replace the old function with the new mutation
            *target = Item::Fn(new_fn.unwrap());

            return;
        }

        panic!("{}", error_spanned!("#[test_case]: {} is not a function", target));
    }
}

impl_unique_arg!(ArgWith);
impl_to_tokens_wrapped!(ArgWith: collection);

#[derive(Clone)]
pub struct ArgVerbatim(pub TokenStream);

impl Parse for ArgVerbatim {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut tokens: TokenStream = TokenStream::new();
        while let Ok(token) = input.parse::<TokenTree>() {
            token.to_tokens(&mut tokens);
        }

        if !input.is_empty() {
            return Err(input.error("verbatim(): Unexpected token"));
        }

        Ok(Self(tokens))
    }
}

impl Mutate for ArgVerbatim {
    fn mutate(self, target: &mut Item) {
        if let Item::Fn(function) = target {
            for stmt in &mut function.block.stmts {
                // This could be optimized
                let tokens = stmt.to_token_stream().to_string();
                let new_stmt = syn::parse_str::<Stmt>(&tokens.replace("r#verbatim", &self.0.to_string()));
                match new_stmt {
                    Ok(parsed) => *stmt = parsed,
                    Err(e) => panic!("{}", error_spanned!("verbatim(): Error modifying statement {}:\n{}", stmt, &e.to_compile_error()))
                };
            }

            return;
        }

        panic!("{}", error_spanned!("#[test_case]: {} is not a function", target));
    }
}

impl_unique_arg!(ArgVerbatim);
impl_to_tokens_wrapped!(ArgVerbatim);