use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Data, DataStruct, Field, Fields, Lit, Meta,
    Path, Type, TypePath,
};

use heck::ToSnakeCase;

use crate::{
    models::{self, TableInfo},
    util::parse_attrs,
};

/// Method to derive an [CreateTable](caisin::CreateTable)
pub fn expand_create_table(
    ident: Ident,
    data: Data,
    attrs: Vec<Attribute>,
) -> syn::Result<TokenStream> {
    let struct_name = ident.clone();
    // println!("ident========={:#?}", ident);
    // println!("data=========={:#?}", data);
    // println!("attrs=========={:#?}", attrs);
    let ret_map = parse_attrs(&attrs, "caisin");
    // println!("ret_map=========={:#?}", ret_map);

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

    let mut tb_info = TableInfo {
        name: struct_name.to_string().to_snake_case(),
        comment: "".to_string(),
        idxs: vec![],
        pks: vec![],
        fields: vec![],
    };
    for (k, v) in ret_map {
        match k.as_str() {
            "tbName" => {
                tb_info.name = match v {
                    Lit::Str(c) => c.token().to_string().replace("\"", ""),
                    _ => "".to_owned(),
                }
            }
            "comment" => {
                tb_info.comment = match v {
                    Lit::Str(c) => c.token().to_string().replace("\"", ""),
                    _ => "".to_owned(),
                }
            }
            _ => {}
        }
    }
    for ele in fields.to_owned() {
        let a = format_ident!("{}", ele.ident.unwrap());
        // println!("ele.ty======{:#?}", ele.ty);
        let mut tb_field = models::Field {
            name: a.to_string(),
            comment: "".to_string(),
            def_value: "".to_string(),
            auto_inc: false,
            null_able: true,
            is_pk: false,
            is_idx: false,
            db_type: "".to_string(),
            size: 0,
        };
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
                    tb_field.null_able = false;
                    idt
                }
            }
            _ => todo!(),
        };
        tb_field.db_type = col_typ.to_string();
        let ret_map = parse_attrs(&ele.attrs, "caisin");

        for (k, v) in ret_map {
            match k.as_str() {
                "comment" => {
                    tb_field.comment = match v {
                        Lit::Str(c) => c.token().to_string().replace("\"", ""),
                        _ => "".to_string(),
                    };
                }
                "def_value" => {
                    tb_field.def_value = match v {
                        Lit::Str(c) => c.token().to_string(),
                        _ => "".to_string(),
                    };
                }
                "index" => {
                    tb_field.is_idx = true;
                }
                "auto_incr" => {
                    tb_field.auto_inc = true;
                }
                "pk" => {
                    tb_field.is_pk = true;
                }
                "size" => {
                    tb_field.size = match v {
                        Lit::Int(c) => {
                            let size: i32 = c.token().to_string().parse().expect("size 不是数字");
                            size
                        }
                        _ => 0,
                    };
                }
                _ => {}
            }
        }
        tb_info.add_field(tb_field.to_owned());
    }

    let sql=tb_info.create_table_sql();
    println!( r#"{}"#,sql);
    Ok(quote!(
    impl #struct_name {
        pub fn create_table() {
            println!("create tabe fun");
        }
     }
    ))
}
