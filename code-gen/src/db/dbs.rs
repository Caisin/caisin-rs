use super::super::models::{Filed, Table};
use rbatis::Rbatis;

#[sql(r#"SELECT TABLE_NAME as name,TABLE_COMMENT as comment FROM information_schema.TABLES WHERE table_schema= ?"#)]
pub async fn get_tables(rb: &Rbatis, schema: &str) -> rbatis::Result<Vec<Table>> {
    impled!()
}

#[py_sql("show FULL COLUMNS from ${table}")]
pub async fn get_filed(rb: &Rbatis, table: &str) -> rbatis::Result<Vec<Filed>> {
    impled!()
}

/// 初始化数据源
pub fn init_db(db_url: &String) -> Rbatis {
    let rb = Rbatis::new();
    rb.init(rbdc_mysql::driver::MysqlDriver {}, db_url).unwrap();
    rb
}

pub async fn get_table_infos(rb: &Rbatis, schema: &str) -> Vec<Table> {
    let tbs_ret = get_tables(&rb, schema).await;
    match tbs_ret {
        Ok(mut tbs) => {
            for tb in tbs.iter_mut() {
                let fileds = get_filed(&rb, tb.name.as_str()).await;
                if let Ok(res) = fileds {
                    tb.fields = Some(res);
                }
            }
            tbs
        }
        Err(e) => {
            println!("{:#?}", e);
            panic!("{}", e);
            // vec![]
        }
    }
}

pub fn get_db_from_url(url: String) -> String {
    let mut ed = url.len();
    match url.find("?") {
        Some(idx) => {
            ed = idx;
        }
        None => {}
    }
    let url = &url[0..ed];
    match url.split("/").last() {
        Some(db) => db.to_string(),
        None => "".to_string(),
    }
}
