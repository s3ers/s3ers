use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::util::import_s3ers_serde;

pub fn expand_serialize_as_ref_str(ident: &Ident) -> syn::Result<TokenStream> {
    let s3ers_serde = import_s3ers_serde();

    Ok(quote! {
        #[automatically_derived]
        impl #s3ers_serde::exports::serde::ser::Serialize for #ident {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: #s3ers_serde::exports::serde::ser::Serializer,
            {
                ::std::convert::AsRef::<::std::primitive::str>::as_ref(self).serialize(serializer)
            }
        }
    })
}
