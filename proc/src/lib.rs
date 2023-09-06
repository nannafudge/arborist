mod length;
mod collection;
mod interpolate;

#[cfg(feature = "test_utils")]
mod testsuite;

use collection::ImplInsertable;
use interpolate::Interpolate;
use length::{
    ImplLength, LengthOverride
};

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::Parse, parse_macro_input,
    DeriveInput, Generics
};

pub(crate) fn extract_impl_generics(generics: Option<&Generics>) -> TokenStream {
    generics.and_then(| generics | {
        let (impl_generics, _, _) = generics.split_for_impl();
        Some(impl_generics.to_token_stream())
    }).unwrap_or_default()
}

pub(crate) fn extract_ty_and_where_generics(generics: Option<&Generics>) -> (TokenStream, TokenStream) {
    generics.and_then(| generics | {
        let (_, ty_generics, where_clause) = generics.split_for_impl();
        Some((ty_generics.to_token_stream(), where_clause.to_token_stream()))
    }).unwrap_or_default()
}

#[proc_macro_attribute]
pub fn length_method(_: proc_macro::TokenStream, target: proc_macro::TokenStream) -> proc_macro::TokenStream {
    target
}

#[proc_macro]
pub fn impl_length(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed: ImplLength = parse_macro_input!(input as ImplLength);
    length::render_impl(parsed, LengthOverride::None)
}

#[proc_macro_derive(Length)]
pub fn derive_length(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed: DeriveInput = parse_macro_input!(input as DeriveInput);

    let method_attr_idx: usize = parsed.attrs.binary_search_by(| attr | {
        let name: String = attr.path().get_ident().and_then(|attr| Some(attr.to_string())).unwrap_or_default();
        return name.as_str().cmp("len_method");
    }).unwrap_or_default();

    let method: LengthOverride = parsed.attrs.get(method_attr_idx).and_then(|attr| {
        attr.parse_args_with(LengthOverride::parse).ok()
    }).unwrap_or_default();

    let (impl_generics, ty_generics, where_clause) = parsed.generics.split_for_impl();

    length::render_impl(
        ImplLength {
            impl_generics: impl_generics.to_token_stream(),
            ty_generics: ty_generics.to_token_stream(),
            where_clause: where_clause.to_token_stream(),
            name: parsed.ident.to_token_stream()
        },
        method
    )
}

#[proc_macro]
pub fn impl_insertable_collection(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed: ImplInsertable = parse_macro_input!(input as ImplInsertable);
    collection::render_impl_insertable(parsed)
}

#[proc_macro_derive(InsertableCollection)]
pub fn derive_insertable_collection(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed: DeriveInput = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = parsed.generics.split_for_impl();

    collection::render_impl_insertable(
        ImplInsertable {
            impl_generics: impl_generics.to_token_stream(),
            ty_generics: ty_generics.to_token_stream(),
            where_clause: where_clause.to_token_stream(),
            name: parsed.ident
        }
    )
}

#[cfg(feature = "test_utils")]
#[proc_macro]
pub fn impl_test(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed: testsuite::ImplTest = parse_macro_input!(input as testsuite::ImplTest);
    testsuite::render_impl(parsed)
}

#[proc_macro]
pub fn interpolate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed: Interpolate = parse_macro_input!(input as Interpolate);

    let mut out = parsed.template.to_token_stream().to_string();
    for (key, value) in parsed.vals {
        out = out.replace(&format!("#[{}]", key), &value.to_string());
    }

    out.parse().expect("Invalid template")
}