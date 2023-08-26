use syn::{Generics, Type, Result, Token};
use syn::parse::{ParseStream, Parse};

#[derive(Clone)]
pub(crate) struct ImplLength {
    pub generics: Generics,
    pub ty: Type
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