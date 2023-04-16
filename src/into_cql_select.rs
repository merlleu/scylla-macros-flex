use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

/// #[derive(IntoCQLSelect)] derives IntoCQLSelect for struct
/// Works only on simple structs without generics etc
pub fn into_cql_select_derive(tokens_input: TokenStream) -> TokenStream {
    let (struct_name, struct_fields) =
        crate::parser::parse_struct_with_named_fields(tokens_input, "FromRow");

    // Generates tokens for field_name: field_type::from_cql(vals_iter.next().ok_or(...)?), ...
    let set_fields_code = struct_fields.named.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;

        quote_spanned! {field.span() =>
            #field_name: {
                let (col_ix, col_value) = vals_iter
                    .next()
                    .unwrap(); // vals_iter size is checked before this code is reached, so
                               // it is safe to unwrap

                match <#field_type as FromCqlVal<Option<CqlValue>>>::from_cql(col_value) {
                    Err(FromCqlValError::ValIsNull) => <#field_type>::default(),
                    Err(e) => return Err(FromRowError::BadCqlVal {
                        err: e,
                        column: col_ix,
                    }),
                    Ok(cql_val) => cql_val
                }
            },
        }
    });

    let generated = quote! {
        impl IntoCQLSelect for #struct_name {
            pub fn into_cql_select() -> &'static str {
                ""
            }
        }
    };

    TokenStream::from(generated)
}


pub trait IntoCQLSelect {
    fn into_cql_select() -> &'static str;
}