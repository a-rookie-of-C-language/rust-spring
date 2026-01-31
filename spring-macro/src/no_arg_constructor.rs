use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::{Fields, ItemStruct};

pub fn no_arg_constructor_impl(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    match expand_no_arg_constructor(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn expand_no_arg_constructor(input: &ItemStruct) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &input.ident;
    match &input.fields {
        Fields::Named(_) => {}
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "no_arg_constructor only supports named fields",
            ))
        }
    };
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    Ok(quote! {
        #input

        impl #impl_generics #ident #ty_generics #where_clause {
            pub fn new_no_args() -> Self {
                Self::default()
            }
        }
    })
}
