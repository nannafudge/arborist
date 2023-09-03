mod length;
mod interpolate;

use length::{ImplLength, LengthOverride};
use interpolate::Interpolate;

use proc_macro2::TokenStream;
use syn::{
    parse_macro_input, DeriveInput,
    parse::Parse
};
use quote::ToTokens;

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
    let (impl_generics, ty_generics, _) = parsed.generics.split_for_impl();
    let mut ty_raw: TokenStream = TokenStream::new();
    parsed.ident.to_tokens(&mut ty_raw);
    ty_generics.to_tokens(&mut ty_raw);

    let method_attr_idx: usize = parsed.attrs.binary_search_by(| attr | {
        let name: String = attr.path().get_ident().and_then(|attr| Some(attr.to_string())).unwrap_or_default();
        return name.as_str().cmp("len_method");
    }).unwrap_or_default();

    let method: LengthOverride = parsed.attrs.get(method_attr_idx).and_then(|attr| {
        attr.parse_args_with(LengthOverride::parse).ok()
    }).unwrap_or_default();

    length::render_impl(
        ImplLength {
            generics: impl_generics.to_token_stream(),
            ty: syn::parse2(ty_raw).expect("Invalid type")
        },
        method
    )
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