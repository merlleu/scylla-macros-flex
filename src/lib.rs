use proc_macro::TokenStream;

mod from_row;
mod from_user_type;
mod parser;

/// #[derive(FromRow)] derives FromRow for struct
/// Works only on simple structs without generics etc
#[proc_macro_derive(FromRowLax)]
pub fn from_row_derive(tokens_input: TokenStream) -> TokenStream {
    from_row::from_row_derive(tokens_input)
}

/// #[derive(FromUserType)] allows to parse a struct as User Defined Type
/// Works only on simple structs without generics etc
#[proc_macro_derive(FromUserTypeLax)]
pub fn from_user_type_derive(tokens_input: TokenStream) -> TokenStream {
    from_user_type::from_user_type_derive(tokens_input)
}