use proc_macro::TokenStream;
use quote::quote;

pub fn from_json_derive(tokens_input: TokenStream) -> TokenStream {
    let (struct_name, _struct_fields) =
        crate::parser::parse_struct_with_named_fields(tokens_input, "FromJson");

    let generated = quote! {
        impl scylla::cql_to_rust::FromCqlVal<scylla::frame::response::result::CqlValue> for #struct_name {
            fn from_cql(cql_val: scylla::frame::response::result::CqlValue)
            -> Result<Self, scylla::cql_to_rust::FromCqlValError> {
                use scylla::cql_to_rust::FromCqlValError;

                // Interpret CqlValue as CQlValue::UserDefinedType
                match cql_val {
                    CqlValue::Text(buf) => serde_json::from_str::<#struct_name>().map_err(|_| FromCqlValError::BadCqlType),
                    _ => Err(FromCqlValError::BadCqlType),
                }
            }
        }
    };

    TokenStream::from(generated)
}

pub fn into_json_derive(tokens_input: TokenStream) -> TokenStream {
    let (struct_name, _struct_fields) =
        crate::parser::parse_struct_with_named_fields(tokens_input, "IntoJson");

    let generated = quote! {
        impl scylla::frame::value::Value for #struct_name {
            fn serialize(&self, buf: &mut Vec<u8>) -> std::result::Result<(), scylla::frame::value::ValueTooBig> {
                let raw = serde_json::to_string(self).map_err(|_| scylla::frame::value::ValueTooBig)?;

                CqlValue::Text(raw).serialize(buf)
            }
        }
    };

    TokenStream::from(generated)
}
