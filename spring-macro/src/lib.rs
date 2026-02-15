use proc_macro::TokenStream;

mod component;
mod data;
mod getter;
mod setter;
mod accessors;
mod no_arg_constructor;
mod all_args_constructor;

#[proc_macro_attribute]
pub fn component(attribute: TokenStream, item: TokenStream) -> TokenStream {
    component::component_impl(attribute, item)
}

#[proc_macro_derive(Component, attributes(autowired))]
pub fn component_derive(item: TokenStream) -> TokenStream {
    component::component_derive_impl(item)
}

#[proc_macro_attribute]
pub fn data(attribute: TokenStream, item: TokenStream) -> TokenStream {
    data::data_impl(attribute, item)
}

#[proc_macro_attribute]
pub fn getter(attribute: TokenStream, item: TokenStream) -> TokenStream {
    getter::getter_impl(attribute, item)
}

#[proc_macro_attribute]
pub fn setter(attribute: TokenStream, item: TokenStream) -> TokenStream {
    setter::setter_impl(attribute, item)
}

#[proc_macro_attribute]
pub fn no_arg_constructor(attribute: TokenStream, item: TokenStream) -> TokenStream {
    no_arg_constructor::no_arg_constructor_impl(attribute, item)
}

#[proc_macro_attribute]
pub fn all_args_constructor(attribute: TokenStream, item: TokenStream) -> TokenStream {
    all_args_constructor::all_args_constructor_impl(attribute, item)
}
