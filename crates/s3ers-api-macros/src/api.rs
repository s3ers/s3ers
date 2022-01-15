//! Details of the `s3ers_api` procedural macro.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    Attribute, Field, Token, Type,
};

mod metadata;
mod request;
mod response;

use self::{metadata::Metadata, request::Request, response::Response};
use crate::util;

mod kw {
    use syn::custom_keyword;

    custom_keyword!(error);
    custom_keyword!(request);
    custom_keyword!(response);
}

/// The result of processing the `s3ers_api` macro, ready for output back to source code.
pub struct Api {
    /// The `metadata` section of the macro.
    metadata: Metadata,

    /// The `request` section of the macro.
    request: Option<Request>,

    /// The `response` section of the macro.
    response: Option<Response>,

    /// The `error` section of the macro.
    error_ty: Option<Type>,
}

impl Api {
    pub fn expand_all(self) -> TokenStream {
        let s3ers_api = util::import_s3ers_api();
        let http = quote! { #s3ers_api::exports::http };

        let metadata = &self.metadata;
        let description = &metadata.description;
        let method = &metadata.method;
        let name = &metadata.name;
        let path = &metadata.path;
        let authentication: TokenStream = self
            .metadata
            .authentication
            .iter()
            .map(|r| {
                let attrs = &r.attrs;
                let value = &r.value;
                quote! {
                    #( #attrs )*
                    authentication: #s3ers_api::AuthScheme::#value,
                }
            })
            .collect();

        let error_ty = self
            .error_ty
            .map_or_else(|| quote! { #s3ers_api::error::SError }, |err_ty| quote! { #err_ty });

        let request = self.request.map(|req| req.expand(metadata, &error_ty, &s3ers_api));
        let response = self.response.map(|res| res.expand(metadata, &error_ty, &s3ers_api));

        let metadata_doc = format!("Metadata for the `{}` API endpoint.", name.value());

        quote! {
            #[doc = #metadata_doc]
            pub const METADATA: #s3ers_api::Metadata = #s3ers_api::Metadata {
                description: #description,
                method: #http::Method::#method,
                name: #name,
                path: #path,
                #authentication
            };

            #request
            #response

            #[cfg(not(any(feature = "client", feature = "server")))]
            type _SilenceUnusedError = #error_ty;
        }
    }
}

impl Parse for Api {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let metadata: Metadata = input.parse()?;

        let req_attrs = input.call(Attribute::parse_outer)?;
        let (request, attributes) = if input.peek(kw::request) {
            let request = parse_request(input, req_attrs)?;
            let after_req_attrs = input.call(Attribute::parse_outer)?;

            (Some(request), after_req_attrs)
        } else {
            // There was no `request` field so the attributes are for `response`
            (None, req_attrs)
        };

        let response = if input.peek(kw::response) {
            Some(parse_response(input, attributes)?)
        } else if !attributes.is_empty() {
            return Err(syn::Error::new_spanned(
                &attributes[0],
                "attributes are not supported on the error type",
            ));
        } else {
            None
        };

        let error_ty = input
            .peek(kw::error)
            .then(|| {
                let _: kw::error = input.parse()?;
                let _: Token![:] = input.parse()?;

                input.parse()
            })
            .transpose()?;

        Ok(Self { metadata, request, response, error_ty })
    }
}

fn parse_request(input: ParseStream<'_>, attributes: Vec<Attribute>) -> syn::Result<Request> {
    let request_kw: kw::request = input.parse()?;
    let _: Token![:] = input.parse()?;
    let fields;
    braced!(fields in input);

    let fields = fields.parse_terminated::<_, Token![,]>(Field::parse_named)?;

    Ok(Request { request_kw, attributes, fields })
}

fn parse_response(input: ParseStream<'_>, attributes: Vec<Attribute>) -> syn::Result<Response> {
    let response_kw: kw::response = input.parse()?;
    let _: Token![:] = input.parse()?;
    let fields;
    braced!(fields in input);

    let fields = fields.parse_terminated::<_, Token![,]>(Field::parse_named)?;

    Ok(Response { attributes, fields, response_kw })
}
