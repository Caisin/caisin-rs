use heck::ToUpperCamelCase;
use std::io::Write;

use crate::{
    db::dbs::{get_table_infos, init_db},
    models::Table,
};

pub async fn gen_rbatis(db_url: String) {
    print!("{},url is {}", "rbatis gen", db_url);
    let rb = init_db(&db_url);
    let tbs = get_table_infos(&rb, "novel").await;
    let mut mod_str = String::from("pub mod prelude;\n\n");
    let mut prelude_str = String::new();

    for tb in tbs {
        gen_by_table(&tb).await;
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

    let mut file = caisin::files::create_file(format!("src/gencode/{}.rs", "prelude").as_str())
        .expect("create fail");
    file.write_all(prelude_str.as_bytes()).expect("write failed");
    println!("data written to prelude file");

    let mut file = caisin::files::create_file(format!("src/gencode/{}.rs", "mod").as_str())
        .expect("create fail");
    file.write_all(mod_str.as_bytes()).expect("write failed");
    println!("data written to mod file");
}

pub async fn gen_by_table(tb: &Table) {
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
    let mut file = caisin::files::create_file(format!("src/gencode/{}.rs", tb.name).as_str())
        .expect("create fail");
    file.write_all(s.as_bytes()).expect("write failed");
    println!("data written to file");
}
