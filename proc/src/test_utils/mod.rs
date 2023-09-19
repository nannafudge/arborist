use std::collections::BTreeSet;
use proc_macro2::Span;
use syn::{
    Item,
    Result, Error
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

impl<T: Mutate> Mutate for Mutators<T> {
    fn mutate(&self, target: &mut Item) {
        for mutator in self {
            mutator.mutate(target);
        }
    }
}

impl<T: Ord + core::fmt::Debug> InsertUnique<T> for Mutators<T> {
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

trait Mutate {
    fn mutate(&self, target: &mut Item);
}

trait InsertUnique<T> {
    fn insert_unique(&mut self, item: T) -> Result<()>;
}

#[macro_use]
mod macros {
    macro_rules! impl_unique_arg {
        ($target:ident $(< $generic:tt $(, $generics:tt)? >)?) => {
            impl $(< $generic $(, $generics)? >)? PartialEq for $target $(<$generic $(, $generics)?>)? {
                fn eq(&self, _: &Self) -> bool { true }
            }
            
            impl $(<$generic $(, $generics)?>)? Eq for $target $(<$generic $(, $generics)?>)? {

            }

            impl $(<$generic $(, $generics)?>)? PartialOrd for $target $(<$generic $(, $generics)?>)? {
                fn partial_cmp(&self, _: &Self) -> Option<core::cmp::Ordering> {
                    Some(core::cmp::Ordering::Equal)
                }
            }
            
            impl $(<$generic $(, $generics)?>)? Ord for $target $(<$generic $(, $generics)?>)? {
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