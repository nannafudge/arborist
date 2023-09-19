use proc_macro2::{
    TokenStream, Ident
};
use quote::{
    quote, ToTokens
};
use syn::{
    Generics, Result,
    parse::{
        ParseStream, Parse
    }
};
use crate::common::{
    extract_impl_generics,
    extract_ty_and_where_generics
};

#[derive(Clone)]
pub(crate) struct ImplInsertable {
    pub impl_generics: TokenStream,
    pub ty_generics: TokenStream,
    pub where_clause: TokenStream,
    pub name: TokenStream
}

impl Parse for ImplInsertable {
    fn parse(input: ParseStream) -> Result<Self> {
        let impl_generics = extract_impl_generics(input.parse::<Generics>().ok().as_ref());
        let name = if let Ok(ident) = input.parse::<Ident>() {
            ident.to_token_stream()
        } else {
            input.parse::<syn::Type>()?.to_token_stream()
        };

        let (ty_generics, where_clause) = extract_ty_and_where_generics(input.parse::<Generics>().ok().as_ref());
        Ok(Self {
            impl_generics: impl_generics,
            ty_generics: ty_generics,
            where_clause: where_clause,
            name: name
        })
    }
}


pub(crate) fn render_impl_insertable(parsed: ImplInsertable) -> TokenStream {
    let impl_generics = parsed.impl_generics;
    let ty_generics = parsed.ty_generics;
    let where_clause = parsed.where_clause;
    let name = parsed.name;

    let (new, capacity) = match name.to_string().as_str() {
        "ArrayVec" | "SliceVec" => (
            quote!{
                let out = #name::new();
                assert!(out.capacity() > 1, "Attempted to create collection with 0 capacity");

                out
            },
            quote!{
                #name::capacity(self) > self.length()
            }
        ),
        _ => (
            quote!{
                #name::with_capacity(1)
            },
            quote!{
                #name::capacity(self) < usize::MAX
            }
        )
    };

    let expanded: proc_macro2::TokenStream = quote! {
        impl #impl_generics InsertableCollection for #name #ty_generics #where_clause {
            fn new() -> Self {
                #new
            }

            fn insert(&mut self, index: usize, item: Self::Output) {
                #name::insert(self, index, item);
            }
        
            fn remove(&mut self, index: usize) -> Self::Output {
                #name::remove(self, index)
            }

            fn set_length(&mut self, length: usize) {
                unsafe { #name::set_len(self, length) }
            }

            fn split_off(&mut self, at: usize) -> Self {
                #name::split_off(self, at)
            }

            fn has_capacity(&self) -> bool {
                #capacity
            }
        }
    };

    expanded
}