use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};

pub fn component_impl(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let input_struct = match &input {
        ItemStruct { ident, .. } => ident,
        _ => panic!("Component can only be used on structs"),
    };
    
    TokenStream::new()
}