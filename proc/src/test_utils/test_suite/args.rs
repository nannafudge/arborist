use crate::common::error_spanned;
use super::{
    Mutate,
    impl_unique_arg,
    impl_to_tokens_wrapped
};
use syn::{
    Item, Stmt
};

#[derive(Clone)]
pub struct ArgSetup(pub Vec<Stmt>);

impl Mutate for ArgSetup {
    fn mutate(mut self, target: &mut Item) {
        if let Item::Fn(function) = target {
            self.0.reverse();
            for stmt in self.0 {
                function.block.stmts.insert(0, stmt);
            }

            return;
        }

        panic!("{}", error_spanned!("#[setup]: {} is not a function", target));
    }
}

impl_unique_arg!(ArgSetup);
impl_to_tokens_wrapped!(ArgSetup: collection);

#[derive(Clone)]
pub struct ArgTeardown(pub Vec<Stmt>);

impl Mutate for ArgTeardown {
    fn mutate(self, target: &mut Item) {
        if let Item::Fn(function) = target {
            function.block.stmts.extend(self.0);

            return;
        }

        panic!("{}", error_spanned!("#[teardown]: {} is not a function", target));
    }
}

impl_unique_arg!(ArgTeardown);
impl_to_tokens_wrapped!(ArgTeardown: collection);