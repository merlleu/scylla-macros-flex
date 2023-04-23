use proc_macro::TokenStream;
use syn::{parse, Data, DeriveInput, Fields, FieldsNamed, Ident};

/// Parses the tokens_input to a DeriveInput and returns the struct name from which it derives and
/// the named fields
pub(crate) fn parse_struct_with_named_fields(
    tokens_input: TokenStream,
    current_derive: &str,
) -> (Ident, FieldsNamed, ScyllaAttributes) {
    let input = parse::<DeriveInput>(tokens_input).expect("No DeriveInput");
    let struct_name = input.ident;
    
    let struct_fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(named_fields) => named_fields,
            _ => panic!("derive({}) works only on structs with named fields!", current_derive),
        },
        _ => panic!("derive({}) works only on structs!", current_derive),
    };

    // Parse attributes
    let attr = ScyllaAttributes::parse(&input.attrs);



    (struct_name, struct_fields, attr)
}



pub(crate) fn parse_struct_name(tokens_input: TokenStream) -> Ident {
    let input = parse::<DeriveInput>(tokens_input).expect("No DeriveInput");
    let struct_name = input.ident;

    struct_name
}

#[derive(Default)]
pub struct ScyllaAttributes {
    pub table: Option<String>,
}

impl ScyllaAttributes {
    pub fn parse(attrs: &[syn::Attribute]) -> Self {
        let mut result = Self::default();

        for attr in attrs {
            if attr.path.is_ident("scylla") {
                let meta = attr.parse_meta().expect("Failed to parse meta");
                match meta {
                    syn::Meta::List(list) => {
                        for nested_meta in list.nested {
                            match nested_meta {
                                syn::NestedMeta::Meta(meta) => {
                                    if meta.path().is_ident("table") {
                                        if let syn::Meta::NameValue(name_value) = meta {
                                            if let syn::Lit::Str(lit_str) = name_value.lit {
                                                result.table = Some(lit_str.value());
                                            }
                                        }
                                    }
                                }
                                _ => panic!("Unexpected meta"),
                            }
                        }
                    }
                    _ => panic!("Unexpected meta"),
                }
            }
        }

        result
    }
}