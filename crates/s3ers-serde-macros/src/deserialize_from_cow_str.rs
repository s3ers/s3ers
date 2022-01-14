use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::util::import_s3ers_serde;

pub fn expand_deserialize_from_cow_str(ident: &Ident) -> syn::Result<TokenStream> {
    let s3ers_serde = import_s3ers_serde();

    Ok(quote! {
        impl<'de> #s3ers_serde::exports::serde::de::Deserialize<'de> for #ident {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: #s3ers_serde::exports::serde::de::Deserializer<'de>,
            {
                type CowStr<'a> = ::std::borrow::Cow<'a, ::std::primitive::str>;

                let cow = #s3ers_serde::deserialize_cow_str(deserializer)?;
                Ok(::std::convert::From::<CowStr<'_>>::from(cow))
            }
        }
    })
}
