use crate::dbs::get_table_infos;
use crate::{models::Table, Args};
use caisin::dbs::{get_db_from_url,  init_db};
use caisin::files::create_file;
use heck::ToUpperCamelCase;
use std::{io::Write, str::FromStr};

pub async fn gen_rbatis(args: &Args) {
    let db_url = args.db_url.to_owned();
    let rb = init_db(&db_url).await.unwrap();
    let db_name = get_db_from_url(db_url);
    let tbs = get_table_infos(&rb, db_name.as_str()).await;
    //生成entity
    gen_entitys(&tbs, args).await;
    gen_mapper(&tbs, args).await;
    gen_mod(args).await;
}

pub async fn gen_mapper(tbs: &Vec<Table>, args: &Args) {
    let mut mod_str = String::new();

    for tb in tbs {
        gen_mapper_by_table(tb, args).await;
        mod_str.push_str(format!("pub mod {};\n", tb.name).as_str());
    }

    println!("data written to prelude file");

    let mut file = create_file(format!("{}/{}/{}.rs", args.out_path, "mapper", "mod").as_str())
        .expect("create fail");
    file.write_all(mod_str.as_bytes()).expect("write failed");
}

pub async fn gen_entitys(tbs: &Vec<Table>, args: &Args) {
    let mut mod_str = String::from("pub mod prelude;\npub use prelude::*;\n\n");
    let mut prelude_str = String::new();

    for tb in tbs {
        gen_entity_by_table(tb, args).await;
        mod_str.push_str(format!("pub mod {};\n", tb.name).as_str());
        prelude_str.push_str(
            format!(
                "pub use super::{}::{};\n",
                tb.name,
                tb.name.to_upper_camel_case()
            )
            .as_str(),
        );
    }

    let mut file = create_file(format!("{}/{}/{}.rs", args.out_path, "entity", "prelude").as_str())
        .expect("create fail");
    file.write_all(prelude_str.as_bytes())
        .expect("write failed");
    println!("data written to prelude file");

    let mut file = create_file(format!("{}/{}/{}.rs", args.out_path, "entity", "mod").as_str())
        .expect("create fail");
    file.write_all(mod_str.as_bytes()).expect("write failed");
    println!("data written to mod file");
}

pub async fn gen_entity_by_table(tb: &Table, args: &Args) {
    let mut tb = tb.clone();
    tb.pre_gen();
    let mut s = String::new();
    //

    s.push_str("use caisin_macros::CreateTable;\n");
    s.push_str("use serde::{Deserialize, Serialize};\n");
    s.push_str("use rbatis::rbdc::datetime::FastDateTime;\n");
    s.push_str("\n");
    s.push_str("#[derive(Clone, Debug, CreateTable, Serialize, Deserialize)]\n");
    s.push_str(
        format!(
            "#[caisin(table_name = \"{}\", comment = \"{}\")]\n",
            tb.name, tb.comment
        )
        .as_str(),
    );
    s.push_str(format!("pub struct {} ", tb.name.to_upper_camel_case()).as_str());
    s.push_str("{\n");
    let f = &tb.fields_ident.unwrap();
    s.push_str(f.as_str());
    s.push_str("}\n");
    let mut file = create_file(format!("{}/{}/{}.rs", args.out_path, "entity", tb.name).as_str())
        .expect("create fail");
    file.write_all(s.as_bytes()).expect("write failed");
    println!("data written to file");
}

pub async fn gen_mapper_by_table(tb: &Table, args: &Args) {
    let tb = tb.clone();
    // tb.pre_gen();
    let mut s = String::new();
    //use super::super::entity::sys_api_db::SysApiDb;

    s.push_str(
        format!(
            "use super::super::entity::{};\n\n",
            tb.name.to_upper_camel_case()
        )
        .as_str(),
    );
    s.push_str(format!("crud!({} {});\n", tb.name.to_upper_camel_case(), "{}").as_str());

    let mut file = create_file(format!("{}/{}/{}.rs", args.out_path, "mapper", tb.name).as_str())
        .expect("create fail");
    file.write_all(s.as_bytes()).expect("write failed");
    println!("data written to file");
}

pub async fn gen_mod(args: &Args) {
    let s = String::from_str(
        r#"
    pub mod entity;
    pub mod mapper;
    "#,
    )
    .unwrap();

    let mut file = create_file(format!("{}/mod.rs", args.out_path).as_str()).expect("create fail");
    file.write_all(s.as_bytes()).expect("write failed");
    println!("data written to file");
}
