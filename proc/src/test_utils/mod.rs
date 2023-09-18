use std::collections::BTreeSet;
use proc_macro2::{
    TokenStream, Span
};
use syn::{
    Token, Item,
    Result, Error,
    parse::{
        Parse, ParseStream
    }
};

mod mocks;
mod test_case;
mod test_suite;

pub use mocks::get_mock;
pub use test_case::{
    TestCase, render_test_case
};
pub use test_suite::{
    TestSuite, render_test_suite
};

type Mutators<T> = BTreeSet<T>;
type Printers<T> = BTreeSet<T>;

trait InsertUnique<T> {
    fn insert_unique(&mut self, item: T) -> Result<()>;
}

impl<T: Ord + core::fmt::Debug> InsertUnique<T> for BTreeSet<T> {
    fn insert_unique(&mut self, item: T) -> Result<()> {
        let err: &str = &format!("Duplicate {:?}", &item);

        if !self.insert(item) {
            return Err(Error::new(
                Span::call_site(),
                err
            ));
        }

        Ok(())
    }
}

trait Print {
    fn print(&self, target: &Item, tokens: &mut TokenStream);
}

trait Mutate {
    fn mutate(&self, target: &mut Item);
}

#[macro_use]
mod macros {
    macro_rules! impl_unique_arg {
        ($target:ident $(< $($generics:ty)* >)?) => {
            impl $(<$($generics)*>)? PartialEq for $target $(<$($generics)*>)? {
                fn eq(&self, _: &Self) -> bool { true }
            }
            
            impl $(<$($generics)*>)? Eq for $target $(<$($generics)*>)? {

            }

            impl $(<$($generics)*>)? PartialOrd for $target $(<$($generics)*>)? {
                fn partial_cmp(&self, _: &Self) -> Option<core::cmp::Ordering> {
                    Some(core::cmp::Ordering::Equal)
                }
            }
            
            impl $(<$($generics)*>)? Ord for $target $(<$($generics)*>)? {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.partial_cmp(other).expect(
                        stringify!($target, ": Unexpected ord result")
                    )
                }
            }
        };
    }

    macro_rules! impl_arg_errors {
        ($target:ty $(, $variant:pat => $name:expr )+) => {
            impl core::fmt::Debug for $target {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        $(
                            $variant => {
                                f.debug_tuple($name).finish()
                            },
                        )+
                    }
                }
            }
        }
    }

    pub(crate) use impl_unique_arg;
    pub(crate) use impl_arg_errors;
}