//! Details of the `response` section of the procedural macro.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    punctuated::Punctuated, spanned::Spanned, Attribute, Field, Ident, Token,
};

use super::{kw, metadata::Metadata};

/// The result of processing the `response` section of the macro.
pub(crate) struct Response {
    /// The `response` keyword
    pub(super) response_kw: kw::response,

    /// The attributes that will be applied to the struct definition.
    pub attributes: Vec<Attribute>,

    /// The fields of the response.
    pub fields: Punctuated<Field, Token![,]>,
}

impl Response {
    pub(super) fn expand(
        &self,
        metadata: &Metadata,
        error_ty: &TokenStream,
        s3ers_api: &TokenStream,
    ) -> TokenStream {
        let s3ers_api_macros = quote! { #s3ers_api::exports::s3ers_api_macros };
        let s3ers_serde = quote! { #s3ers_api::exports::s3ers_serde };

        let docs = format!(
            "Data in the response from the `{}` API endpoint.",
            metadata.name.value()
        );
        let struct_attributes = &self.attributes;

        let response_ident = Ident::new("Response", self.response_kw.span());
        let fields = &self.fields;
        quote! {
            #[doc = #docs]
            #[derive(
                Clone,
                Debug,
                #s3ers_api_macros::Response,
                #s3ers_serde::Outgoing,
                #s3ers_serde::_FakeDeriveSerde,
            )]
            #[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
            #[incoming_derive(!Deserialize, #s3ers_api_macros::_FakeDeriveS3ers)]
            #[s3ers_api(error_ty = #error_ty)]
            #( #struct_attributes )*
            pub struct #response_ident {
                #fields
            }
        }
    }
}
