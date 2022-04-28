use proc_macro::TokenStream;
use quote::*;
use syn::{ parse_macro_input, ItemStruct};
use std::collections::BTreeMap;


/// Derives a signature generation on the fields of a struct
#[proc_macro_derive(BybitSignable)]
pub fn sample_macro(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let ItemStruct {
        attrs: _,
        vis: _,
        struct_token: _,
        ident,
        generics: _,
        fields,
        semi_token: _
    } = input;
    let tokens = quote! {
        // Replacement code defined here
    };
    return TokenStream::from(tokens);
}