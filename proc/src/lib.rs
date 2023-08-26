mod length;
mod interpolate;

use length::ImplLength;
use interpolate::InterpolateExpr;

use syn::{Expr, parse_macro_input};
use quote::{ToTokens, quote};

#[proc_macro]
pub fn impl_length(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed: ImplLength = parse_macro_input!(input as ImplLength);
    // Done to avoid implementing quote.rs traits for ImplTrait
    let (generics, _, _) = parsed.generics.split_for_impl();
    let ty = parsed.ty;
    let expanded: proc_macro2::TokenStream = quote! {
        impl #generics Length for #ty {
            fn length(&self) -> usize {
                self.len()
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(Length)]
pub fn derive_length(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_length(input)
}

#[proc_macro]
pub fn interpolate_expr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed: InterpolateExpr = parse_macro_input!(input as InterpolateExpr);

    let mut out = parsed.template.to_token_stream().to_string();
    for (key, value) in &parsed.vals {
        out = out.replace(&format!("$[{}]", key), &value.to_string());
    }

    let expanded: Expr = syn::parse_str(&out).expect("Invalid template");
    proc_macro::TokenStream::from(expanded.into_token_stream())
}