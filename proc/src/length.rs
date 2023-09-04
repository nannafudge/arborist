use proc_macro2::TokenStream;
use quote::{
    ToTokens, quote
};
use syn::{
    Result, Generics, Ident,
    parse::{ParseStream, Parse},
};
use super::{
    extract_impl_generics,
    extract_ty_and_where_generics
};

#[derive(Clone)]
pub(crate) struct ImplLength {
    pub impl_generics: TokenStream,
    pub ty_generics: TokenStream,
    pub where_clause: TokenStream,
    pub name: TokenStream,
}

impl Parse for ImplLength {
    fn parse(input: ParseStream) -> Result<Self> {
        let impl_generics = extract_impl_generics(input.parse::<Generics>().ok().as_ref());
        let name: TokenStream = if let Ok(ident) = input.parse::<Ident>() {
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

#[derive(Default)]
pub(crate) enum LengthOverride {
    #[default] None,
    Some(String)
}

impl Parse for LengthOverride {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(LengthOverride::from(
            input.parse::<syn::LitStr>()?.value()
        ))
    }
}

impl ToTokens for LengthOverride {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let method: &str = match self {
            LengthOverride::Some(custom) => custom,
            LengthOverride::None => "self.len()"
        };

        syn::parse_str::<syn::Expr>(method)
            .expect("Error creating len method invocation")
            .to_tokens(tokens)
    }
}

impl From<String> for LengthOverride {
    fn from(value: String) -> Self {
        match value.as_str() {
            "" => LengthOverride::None,
            _ => LengthOverride::Some(value)
        }
    }
}

// Type generics are embedded within the Type definition itself
pub(crate) fn render_impl(parsed: ImplLength, method: LengthOverride) -> proc_macro::TokenStream {
    let impl_generics = parsed.impl_generics;
    let ty_generics = parsed.ty_generics;
    let where_clause = parsed.where_clause;
    let name = parsed.name;

    let expanded: proc_macro2::TokenStream = quote! {
        impl #impl_generics Length for #name #ty_generics #where_clause {
            fn length(&self) -> usize {
                #method
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}