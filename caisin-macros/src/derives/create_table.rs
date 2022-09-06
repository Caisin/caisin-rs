use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Data, DataStruct, Field, Fields, Lit, Meta,
    Path, Type, TypePath,
};

use crate::util::parse_attrs;

/// Method to derive an [CreateTable](caisin::CreateTable)
pub fn expand_create_table(
    ident: Ident,
    data: Data,
    attrs: Vec<Attribute>,
) -> syn::Result<TokenStream> {
    let struct_name = ident.clone();
    println!("ident========={:#?}", ident);
    println!("data=========={:#?}", data);
    println!("attrs=========={:#?}", attrs);
    let ret_map = parse_attrs(&attrs, "caisin");
    println!("ret_map=========={:#?}", ret_map);

    let fields = match data {
        Data::Struct(DataStruct {
            fields: Fields::Named(named),
            ..
        }) => named.named,
        _ => {
            return Ok(quote_spanned! {
                ident.span() => compile_error!("you can only derive DeriveActiveModel on structs");
            });
        }
    }
    .into_iter();

    for ele in fields.to_owned() {
        let a = format_ident!("{}", ele.ident.unwrap());
        println!("filed======{}", a);
        // println!("ele.ty======{:#?}", ele.ty);
        let col_typ = match ele.ty {
            Type::Path(p) => {
                let f = p.path.segments.first().expect("col_type error").to_owned();
                let idt = f.ident;
                if idt == "Option" {
                    match f.arguments {
                        syn::PathArguments::AngleBracketed(p) => {
                            let args = p.args.first().expect("no args").to_owned();
                            match args {
                                syn::GenericArgument::Type(t) => match t {
                                    Type::Path(p) => {
                                        let idt = p
                                            .path
                                            .to_owned()
                                            .segments
                                            .first()
                                            .expect("err")
                                            .to_owned()
                                            .ident;
                                        idt
                                    }
                                    _ => todo!(),
                                },
                                _ => todo!(),
                            }
                        }
                        _ => todo!(),
                    }
                    // println!("{}", )
                } else {
                    idt
                }
            }
            _ => todo!(),
        };
        println!("col_typ==={}", col_typ);
        let ret_map = parse_attrs(&ele.attrs, "caisin");
        println!("ret_map=========={:#?}", ret_map);
    }

    Ok(quote!(
    impl #struct_name {
        pub fn create_table() {
            println!("create tabe fun");
        }
     }
    ))
}
