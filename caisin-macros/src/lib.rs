mod derives;
mod models;
mod util;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CreateTable, attributes(caisin))]
pub fn derive_create_table(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = parse_macro_input!(input);

    match derives::expand_create_table(ident, data, attrs) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
