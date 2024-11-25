extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Representable)]
pub fn derive_displayable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let gen = quote! {
        impl representable_interface::Representable for #name {
            fn represent(&self) -> String {
                format!("{:?}", self)
            }
        }
    };
    gen.into()
}
