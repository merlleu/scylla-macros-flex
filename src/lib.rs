use proc_macro::TokenStream;

mod from_row;
mod from_user_type;
mod into_cql_select;
mod parser;
use into_cql_select::SelectCQLQuery;

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

#[proc_macro_derive(IntoCQLSelect)]
pub fn into_cql_select_derive(tokens_input: TokenStream) -> TokenStream {
    into_cql_select::into_cql_select_derive(tokens_input)
}

cfg_if::cfg_if! {
    if #[cfg(feature = "speedy")] {
        mod ext_speedy;
        /// #[derive(FromSpeedy)] derives FromSpeedy for structs/enum
        #[proc_macro_derive(FromSpeedy)]
        pub fn from_speedy_derive(tokens_input: TokenStream) -> TokenStream {
            ext_speedy::from_speedy_derive(tokens_input)
        }

        /// #[derive(IntoSpeedy)] derives FromSpeedy for structs/enum
        #[proc_macro_derive(IntoSpeedy)]
        pub fn into_speedy_derive(tokens_input: TokenStream) -> TokenStream {
            ext_speedy::into_speedy_derive(tokens_input)
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "serde_json")] {
        mod ext_serde_json;
        /// #[derive(FromJson)] derives FromJson for struct
        /// Works only on simple structs without generics etc
        #[proc_macro_derive(FromJson)]
        pub fn from_json_derive(tokens_input: TokenStream) -> TokenStream {
            ext_serde_json::from_json_derive(tokens_input)
        }

        /// #[derive(IntoJson)] derives FromJson for struct
        /// Works only on simple structs without generics etc
        #[proc_macro_derive(IntoJson)]
        pub fn into_json_derive(tokens_input: TokenStream) -> TokenStream {
            ext_serde_json::into_json_derive(tokens_input)
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "rmp-serde")] {
        mod ext_rmp_serde;
        /// #[derive(FromMessagePack)] derives FromMessagePack for struct
        /// Works only on simple structs without generics etc
        #[proc_macro_derive(FromMessagePack)]
        pub fn from_mp_derive(tokens_input: TokenStream) -> TokenStream {
            ext_rmp_serde::from_mp_derive(tokens_input)
        }

        /// #[derive(IntoMessagePack)] derives FromMessagePack for struct
        /// Works only on simple structs without generics etc
        #[proc_macro_derive(IntoMessagePack)]
        pub fn into_mp_derive(tokens_input: TokenStream) -> TokenStream {
            ext_rmp_serde::into_mp_derive(tokens_input)
        }
    }
}
