use proc_macro2::TokenStream;
use syn::{Ident, Item};

mod mocks;
mod tests;
mod suite;

pub use mocks::get_mock;
pub use tests::render_test_case;
pub use tests::TestCase;
pub use suite::TestSuite;

trait Print {
    fn print(&self, target: &Item, tokens: &mut TokenStream);
}

trait Mutate {
    fn mutate(&self, target: &mut Item);
}

pub(crate) fn render_impl_mock(name: Ident) -> proc_macro::TokenStream {
    get_mock(name).into()
}