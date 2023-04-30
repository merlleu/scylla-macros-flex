use proc_macro::TokenStream;
use quote::quote;

/// #[derive(IntoCQLSelect)] derives IntoCQLSelect for struct
/// Works only on simple structs without generics etc
pub fn into_cql_select_derive(tokens_input: TokenStream) -> TokenStream {
    let (struct_name, struct_fields, attrs) =
        crate::parser::parse_struct_with_named_fields(tokens_input, "FromRow");

    let table_name = attrs
        .table
        .unwrap_or_else(|| struct_name.to_string().to_lowercase());

    // gerate sql query
    let mut select_fields = Vec::new();
    for field in struct_fields.named.iter() {
        let field_name = field.ident.as_ref().unwrap();
        select_fields.push(field_name.to_string());
    }

    let select_fields_str = select_fields.join(", ");

    let generated = quote! {
        impl #struct_name {
            pub const fn __cql_select_query<'a>() -> &'a str {
                concat!("SELECT ", #select_fields_str, " FROM ", #table_name)
            }
        }
    };

    TokenStream::from(generated)
}

