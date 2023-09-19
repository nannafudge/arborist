use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
    parse::Parse,
    parse_macro_input,
    DeriveInput
};

mod length;
mod common;
mod collection;
mod interpolate;

#[cfg(feature = "test_utils")]
mod test_utils;

use collection::*;
use interpolate::*;
use length::*;

#[cfg(feature = "test_utils")]
use test_utils::*;

#[proc_macro_attribute]
pub fn length_method(_: TokenStream, target: TokenStream) -> TokenStream {
    target
}

#[proc_macro]
pub fn impl_length(input: TokenStream) -> TokenStream {
    let parsed: ImplLength = parse_macro_input!(input as ImplLength);
    render_length_impl(parsed, LengthOverride::None).into()
}

#[proc_macro_derive(Length)]
pub fn derive_length(input: TokenStream) -> TokenStream {
    let parsed: DeriveInput = parse_macro_input!(input as DeriveInput);

    let method_attr_idx: usize = parsed.attrs.binary_search_by(| attr | {
        let name: String = attr.path().get_ident().and_then(|attr| Some(attr.to_string())).unwrap_or_default();
        return name.as_str().cmp("len_method");
    }).unwrap_or_default();

    let method: LengthOverride = parsed.attrs.get(method_attr_idx).and_then(|attr| {
        attr.parse_args_with(LengthOverride::parse).ok()
    }).unwrap_or_default();

    let (impl_generics, ty_generics, where_clause) = parsed.generics.split_for_impl();

    render_length_impl(
        ImplLength {
            impl_generics: impl_generics.to_token_stream(),
            ty_generics: ty_generics.to_token_stream(),
            where_clause: where_clause.to_token_stream(),
            name: parsed.ident.to_token_stream()
        },
        method
    ).into()
}

#[proc_macro]
pub fn impl_insertable_collection(input: TokenStream) -> TokenStream {
    let parsed: ImplInsertable = parse_macro_input!(input as ImplInsertable);
    render_impl_insertable(parsed).into()
}

#[proc_macro_derive(InsertableCollection)]
pub fn derive_insertable_collection(input: TokenStream) -> TokenStream {
    let parsed: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = parsed.generics.split_for_impl();

    render_impl_insertable(
        ImplInsertable {
            impl_generics: impl_generics.to_token_stream(),
            ty_generics: ty_generics.to_token_stream(),
            where_clause: where_clause.to_token_stream(),
            name: parsed.ident.to_token_stream()
        }
    ).into()
}

#[proc_macro]
pub fn interpolate(input: TokenStream) -> TokenStream {
    let parsed: Interpolate = parse_macro_input!(input as Interpolate);

    let mut out = parsed.template.to_token_stream().to_string();
    for (key, value) in parsed.vals {
        out = out.replace(&format!("#[{}]", key), &value.to_string());
    }

    out.parse().expect("Invalid template")
}

#[cfg(feature = "test_utils")]
#[proc_macro_attribute]
pub fn test_suite(_: TokenStream, target: TokenStream) -> TokenStream {
    let test_suite: test_utils::TestSuite = parse_macro_input!(target as test_utils::TestSuite);
    test_utils::render_test_suite(test_suite).into()
}

#[cfg(feature = "test_utils")]
#[proc_macro_attribute]
pub fn test_case(attr_args: TokenStream, target: TokenStream) -> TokenStream {
    use syn::ItemFn;

    let test_case: TestCase = parse_macro_input!(attr_args as TestCase);
    let test_fn: ItemFn = parse_macro_input!(target as ItemFn);
    render_test_case(test_case, test_fn).into()
}

#[cfg(feature = "test_utils")]
#[proc_macro]
pub fn impl_mock(input: TokenStream) -> TokenStream {
    let parsed: syn::Ident = parse_macro_input!(input as syn::Ident);
    get_mock(parsed).into()
}