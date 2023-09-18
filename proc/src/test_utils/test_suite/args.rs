use super::{
    Mutate,
    impl_unique_arg
};
use syn::{
    Item, Stmt
};

#[derive(Clone)]
pub struct ArgSetup(pub Vec<Stmt>);

impl Mutate for ArgSetup {
    fn mutate(&self, target: &mut Item) {
        if let Item::Fn(function) = target {
            for stmt in self.0.iter().rev() {
                function.block.stmts.insert(0, stmt.to_owned());
            }

            return;
        }

        panic!(
            "ArgSetup.mutate(): expected function, received syn::Item with {:?}",
            core::mem::discriminant(target)
        );
    }
}

impl_unique_arg!(ArgSetup);

#[derive(Clone)]
pub struct ArgTeardown(pub Vec<Stmt>);

impl Mutate for ArgTeardown {
    fn mutate(&self, target: &mut Item) {
        if let Item::Fn(function) = target {
            for stmt in &self.0 {
                function.block.stmts.push(stmt.to_owned());
            }

            return;
        }

        panic!(
            "ArgTeardown.mutate(): expected function, received syn::Item with {:?}",
            core::mem::discriminant(target)
        );
    }
}

impl_unique_arg!(ArgTeardown);