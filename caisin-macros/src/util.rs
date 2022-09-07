use std::collections::HashMap;

use proc_macro2::Span;
use syn::{punctuated::Punctuated, token::Comma, Attribute, Lit, LitBool, Meta};

pub fn parse_attrs(attrs: &Vec<Attribute>, idt: &str) -> HashMap<String, Lit> {
    let mut ret_map = HashMap::new();
    attrs.iter().for_each(|attr| {
        if attr.path.get_ident().map(|i| i == idt) != Some(true) {
            return;
        }
        if let Ok(list) = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated) {
            for meta in list.iter() {
                if let Meta::NameValue(nv) = meta {
                    if let Some(ident) = nv.path.get_ident() {
                        let name = &nv.lit;
                        ret_map.insert(ident.to_string(), name.to_owned());
                    }
                } else if let Meta::Path(path) = meta {
                    if let Some(ident) = path.get_ident() {
                        let v = Lit::Bool(LitBool::new(true, Span::call_site()));
                        ret_map.insert(ident.to_string(), v);
                    }
                }
            }
        }
    });
    ret_map
}
