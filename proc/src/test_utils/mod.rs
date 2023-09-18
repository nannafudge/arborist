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
        let err: &str = &format!("Duplicate item: {:?}", &item);

        if !self.insert(item) {
            return Err(Error::new(
                Span::call_site(),
                err
            ));
        }

        Ok(())
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

trait Print {
    fn print(&self, target: &Item, tokens: &mut TokenStream);
}

trait Mutate {
    fn mutate(&self, target: &mut Item);
}

#[macro_use]
mod macros {
    macro_rules! impl_arg_ord_traits {
        ($target:ty) => {
            impl PartialEq for $target {
                fn eq(&self, other: &Self) -> bool {
                    core::mem::discriminant(self) == core::mem::discriminant(other)
                }
            }
            
            impl Eq for $target {}
            
            impl PartialOrd for $target {
                fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                    ((self as *const $target) as u8).partial_cmp(
                        &((other as *const $target) as u8)
                    )
                }
            }
            
            impl Ord for $target {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.partial_cmp(other).expect(
                        stringify!($target, ": Unexpected ord result")
                    )
                }
            }
        };
        ($target:ty: unique) => {
            impl PartialEq for $target {
                fn eq(&self, other: &Self) -> bool { true }
            }
            impl Eq for $target {}
            
            impl PartialOrd for $target {
                fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                    Some(core::cmp::Ordering::Equal)
                }
            }
            
            impl Ord for $target {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.partial_cmp(other).expect(
                        stringify!($target, ": Unexpected ord result")
                    )
                }
            }
        };
    }

    macro_rules! impl_enum_debug {
        ($target:ty $(, $variant:pat )+) => {
            impl core::fmt::Debug for $target {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        $(
                            $variant => {
                                f.debug_tuple("setup").finish()
                            },
                        )+
                    }
                }
            }
        }
    }

    pub(crate) use impl_arg_ord_traits;
    pub(crate) use impl_enum_debug;
}