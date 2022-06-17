use proc_macro::TokenStream;
use quote::quote;

pub fn from_mp_derive(tokens_input: TokenStream) -> TokenStream {
    let (struct_name, _struct_fields) =
        crate::parser::parse_struct_with_named_fields(tokens_input, "FromMessagePack");

    let generated = quote! {
        impl scylla::cql_to_rust::FromCqlVal<scylla::frame::response::result::CqlValue> for #struct_name {
            fn from_cql(cql_val: scylla::frame::response::result::CqlValue)
            -> Result<Self, scylla::cql_to_rust::FromCqlValError> {
                use scylla::cql_to_rust::FromCqlValError;
                use scylla::frame::response::result::CqlValue;
                match cql_val {
                    CqlValue::Blob(buf) => rmp_serde::from_slice::<#struct_name>(&buf).map_err(|_| FromCqlValError::BadCqlType),
                    _ => Err(FromCqlValError::BadCqlType),
                }
            }
        }
    };

    TokenStream::from(generated)
}

pub fn into_mp_derive(tokens_input: TokenStream) -> TokenStream {
    let (struct_name, _struct_fields) =
        crate::parser::parse_struct_with_named_fields(tokens_input, "IntoMessagePack");

    let generated = quote! {
        impl scylla::frame::value::Value for #struct_name {
            fn serialize(&self, buf: &mut Vec<u8>) -> std::result::Result<(), scylla::frame::value::ValueTooBig> {
                use scylla::frame::response::result::CqlValue;
                let raw = rmp_serde::to_vec(self).map_err(|_| scylla::frame::value::ValueTooBig)?;

                CqlValue::Blob(raw).serialize(buf)
            }
        }
    };

    TokenStream::from(generated)
}
