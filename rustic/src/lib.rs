use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HelloWorld)]
pub fn hello_world_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl HelloWorld for #name {
            fn hello_world() -> String {
                format!("Hello World! from {}", stringify!(#name))
            }
        }
    };

    expanded.into()
}