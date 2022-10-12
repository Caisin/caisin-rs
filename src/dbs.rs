use anyhow::{anyhow, Result};
use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use rbdc_sqlite::driver::SqliteDriver;

/// 初始化数据源
pub async fn init_db(db_url: &str) -> Result<Rbatis> {
    let rb = Rbatis::new();
    let db_type = get_db_type(db_url);
    match db_type as &str {
        "mysql" => {
            rb.init(MysqlDriver {}, db_url).unwrap();
            let ret: Result<i8, rbatis::rbdc::Error> = rb.fetch_decode("select 1", vec![]).await;
            match ret {
                Ok(_) => Ok(rb),
                Err(e) => Err(anyhow!("数据源{db_url}初始化失败!{e}")),
            }
        }
        "sqlite" => {
            rb.init(SqliteDriver {}, db_url).unwrap();
            Ok(rb)
        }
        _ => Err(anyhow!("不支持的数据库类型:{}", db_type)),
    }
}

/// 通过数据库连接url获取连接的数据库名称
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
/// 通过数据库连接url获取数据库类型
pub fn get_db_type(url: &str) -> &str {
    let find = url.find("://").unwrap();
    &url[..find]
}
