use proc_macro2::{TokenStream, Ident};
use quote::quote;
use syn::{
    Generics, Result,
    parse::{ParseStream, Parse}
};
use super::{
    extract_impl_generics,
    extract_ty_and_where_generics
};

#[derive(Clone)]
pub(crate) struct ImplInsertable {
    pub impl_generics: TokenStream,
    pub ty_generics: TokenStream,
    pub where_clause: TokenStream,
    pub name: Ident
}

impl Parse for ImplInsertable {
    fn parse(input: ParseStream) -> Result<Self> {
        let impl_generics = extract_impl_generics(input.parse::<Generics>().ok().as_ref());
        let name: Ident = input.parse()?;
        let (ty_generics, where_clause) = extract_ty_and_where_generics(input.parse::<Generics>().ok().as_ref());
        Ok(Self {
            impl_generics: impl_generics,
            ty_generics: ty_generics,
            where_clause: where_clause,
            name: name
        })
    }
}

pub(crate) fn render_impl_insertable(parsed: ImplInsertable) -> proc_macro::TokenStream {
    let impl_generics = parsed.impl_generics;
    let ty_generics = parsed.ty_generics;
    let where_clause = parsed.where_clause;
    let name = parsed.name;

    let expanded: proc_macro2::TokenStream = quote! {
        impl #impl_generics InsertableCollection for #name #ty_generics #where_clause {
            fn insert(&mut self, index: usize, item: Self::Output) {
                #name::insert(self, index, item);
            }
        
            fn remove(&mut self, index: usize) -> Self::Output {
                #name::remove(self, index)
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}