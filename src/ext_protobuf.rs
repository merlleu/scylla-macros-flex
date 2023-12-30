use proc_macro::TokenStream;
use quote::quote;

pub fn from_protobuf_derive(tokens_input: TokenStream) -> TokenStream {
    let struct_name = crate::parser::parse_struct_name(tokens_input);

    let generated = quote! {
        impl scylla::cql_to_rust::FromCqlVal<scylla::frame::response::result::CqlValue> for #struct_name {
            fn from_cql(cql_val: scylla::frame::response::result::CqlValue)
            -> Result<Self, scylla::cql_to_rust::FromCqlValError> {
                use scylla::cql_to_rust::FromCqlValError;
                use scylla::frame::response::result::CqlValue;
                match cql_val {
                    CqlValue::Blob(buf) => prost::Message::decode(buf.as_slice())
                        .map_err(|_| FromCqlValError::BadCqlType),
                    CqlValue::Empty => Ok(#struct_name::default()),
                    _ => Err(FromCqlValError::BadCqlType),
                }
            }
        }
    };

    TokenStream::from(generated)
}

pub fn into_protobuf_derive(tokens_input: TokenStream) -> TokenStream {
    let struct_name = crate::parser::parse_struct_name(tokens_input);

    let generated = quote! {
        impl scylla::frame::value::Value for #struct_name {
            fn serialize(&self, buf: &mut Vec<u8>) -> std::result::Result<(), scylla::frame::value::ValueTooBig> {
                use scylla::frame::response::result::CqlValue;
                let raw_buf = prost::Message::encode_to_vec(self);
                CqlValue::Blob(raw_buf).serialize(buf)
            }
        }

        impl scylla::serialize::value::SerializeCql for #struct_name {
            fn serialize<'b>(
                &self,
                typ: &scylla::frame::response::result::ColumnType,
                writer: scylla::serialize::CellWriter<'b>,
            ) -> Result<scylla::serialize::writers::WrittenCellProof<'b>, scylla::serialize::SerializationError> {
                use scylla::frame::response::result::CqlValue;
                let raw_buf = prost::Message::encode_to_vec(self);
                CqlValue::Blob(raw_buf).serialize(typ,writer)
            }
        }
    };

    TokenStream::from(generated)
}
