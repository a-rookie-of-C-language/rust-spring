use proc_macro::TokenStream;

mod component;

#[proc_macro_attribute]
pub fn component(attribute: TokenStream, item: TokenStream) -> TokenStream {
    component::component_impl(attribute, item)
}