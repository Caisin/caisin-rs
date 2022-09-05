use std::io::Write;

use crate::{
    db::dbs::{get_table_infos, init_db},
    models::Table,
};

pub async fn gen_rbatis(db_url: String) {
    print!("{},url is {}", "rbatis gen", db_url);
    let rb = init_db(&db_url);
    let tbs = get_table_infos(&rb, "novel").await;
    for tb in tbs {
        gen_by_table(&tb).await
    }
}

pub async fn gen_by_table(tb: &Table) {
    
    let mut file = caisin::files::create_file(format!("gen-code/{}.rs", tb.name).as_str())
        .expect("create fail");
    file.write_all(format!("{:?}", tb.fields).as_bytes())
        .expect("write failed");
    println!("data written to file");
}
