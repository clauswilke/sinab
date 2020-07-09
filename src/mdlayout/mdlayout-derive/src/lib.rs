#![recursion_limit = "128"]

/// Macros that enable `#[derive(...)]` features in the main crate. Originally written by Simon Sapin.

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

include!("style.rs");

// The remainder of this file is a test of proc_macro_derive().

use quote::quote;
use syn::{parse_macro_input, DeriveInput, Result};
use syn::parse::{Parse, ParseStream};


#[proc_macro_derive(StringFormat)]
pub fn string_format_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // get the name of the type we want to implement the trait for
    let name = &input.ident;

    let expanded = quote! {
        impl StringFormat for #name {
            fn string_format(&self) -> String {
                "hello float".to_string()
            }
        }
    };

    TokenStream::from(expanded)
}