
use syn::{
    parse_macro_input, Token,
    Result, Type, Generics, Expr
};
use syn::parse::{Parse, ParseStream};
use quote::quote;

#[derive(Clone)]
struct ImplLength {
    generics: Generics,
    ty: Type
}

#[derive(Clone)]
struct ImplTrait {
    generics: Generics,
    trait_: Type,
    target: Type,
    body: Expr
}

impl Parse for ImplLength {
    fn parse(input: ParseStream) -> Result<Self> {
        let generics: Generics = input.parse()?;
        input.parse::<Token![,]>()?;
        Ok(Self {
            generics: generics,
            ty: input.parse()?
        })
    }
}

impl Parse for ImplTrait {
    fn parse(input: ParseStream) -> Result<Self> {
        let generics: Generics = input.parse()?;
        input.parse::<Token![,]>()?;
        let trait_: Type = input.parse()?;
        input.parse::<Token![,]>()?;
        let target: Type = input.parse()?;
        input.parse::<Token![,]>()?;
        Ok(Self {
            generics: generics,
            trait_: trait_,
            target: target,
            body: input.parse()?
        })
    }
}

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

#[proc_macro]
pub fn impl_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed: ImplTrait = parse_macro_input!(input as ImplTrait);
    // Done to avoid implementing quote.rs traits for ImplTrait
    let (generics, _, _) = parsed.generics.split_for_impl();
    let trait_ = parsed.trait_;
    let target = parsed.target;
    let body = parsed.body;
    let expanded: proc_macro2::TokenStream = quote! {
        impl #generics #trait_ for #target {
            #body
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(Length)]
pub fn derive_length(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_length(input)
}