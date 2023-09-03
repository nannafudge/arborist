use proc_macro2::TokenStream;
use syn::{
    Generics, Type, Result, Token,
    parse::{ParseStream, Parse}
};
use quote::{quote, ToTokens};

#[derive(Clone)]
pub(crate) struct ImplLength {
    pub generics: TokenStream,
    pub ty: Type,
}

impl Parse for ImplLength {
    fn parse(input: ParseStream) -> Result<Self> {
        let generics: Generics = input.parse()?;
        input.parse::<Token![,]>()?;

        Ok(Self {
            generics: generics.to_token_stream(),
            ty: input.parse()?
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

pub(crate) fn render_impl(parsed: ImplLength, method: LengthOverride) -> proc_macro::TokenStream {
    let generics = parsed.generics;
    let ty = parsed.ty;

    let expanded: proc_macro2::TokenStream = quote! {
        impl #generics Length for #ty {
            fn length(&self) -> usize {
                #method
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}