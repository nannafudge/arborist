use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::{
    Ident, Block,
    Attribute
};

mod mocks;
mod suite;
mod tests;

pub use mocks::get_mock;
pub use suite::TestSuite;
pub use tests::TestCase;

pub(crate) fn render_impl_mock(name: Ident) -> proc_macro::TokenStream {
    get_mock(name).into()
}

#[inline]
fn steal<'c, T: ?Sized>(item: &T) -> &'c T {
    unsafe {
        core::mem::transmute::<&T, &'c T>(item)
    }
}

pub fn attribute_name_to_bytes<'c>(attr: &Attribute) -> Option<&'c [u8]> {
    let name: Option<&'c [u8]> = attr.meta.path().get_ident().map(| ident: &syn::Ident | {
        steal(ident.to_string().as_bytes())
    });

    name
}

pub fn block_to_tokens(body: &Block) -> TokenStream {
    let mut out: TokenStream = TokenStream::new();
    out.append_all(&body.stmts);

    out
}