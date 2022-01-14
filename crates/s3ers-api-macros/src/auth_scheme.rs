use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

mod kw {
    syn::custom_keyword!(None);
    syn::custom_keyword!(AwsSignatureV4Header);
    syn::custom_keyword!(AwsSignatureV4QueryParams);
}

pub enum AuthScheme {
    None(kw::None),
    AwsSignatureV4Header(kw::AwsSignatureV4Header),
    AwsSignatureV4QueryParams(kw::AwsSignatureV4QueryParams),
}

impl Parse for AuthScheme {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::None) {
            input.parse().map(Self::None)
        } else if lookahead.peek(kw::AwsSignatureV4Header) {
            input.parse().map(Self::AwsSignatureV4Header)
        } else if lookahead.peek(kw::AwsSignatureV4QueryParams) {
            input.parse().map(Self::AwsSignatureV4QueryParams)
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for AuthScheme {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            AuthScheme::None(kw) => kw.to_tokens(tokens),
            AuthScheme::AwsSignatureV4Header(kw) => kw.to_tokens(tokens),
            AuthScheme::AwsSignatureV4QueryParams(kw) => kw.to_tokens(tokens),
        }
    }
}
