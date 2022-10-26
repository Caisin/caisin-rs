use std::collections::HashMap;

use anyhow::{anyhow, Result};
use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use rbdc_sqlite::driver::SqliteDriver;
use rbs::{to_value, Value};

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
/// 获取map list
pub async fn fetch_map_list(
    rb: &Rbatis,
    sql: &str,
    args: Vec<rbs::Value>,
) -> rbatis::Result<Vec<HashMap<String, rbs::Value>>> {
    rb.fetch_decode(sql, args).await
}

/// 将sql转为in参数
pub fn sql_in<T: serde::Serialize>(
    sql: &str,
    field: &str,
    args: &Vec<Value>,
    in_ids: Vec<T>,
) -> (String, Vec<Value>) {
    let mut els = vec![];
    let mut ret_args = args.clone();
    for ele in in_ids {
        ret_args.push(to_value!(ele));
        els.push("?");
    }
    let mut sql = String::from(sql);
    let join = els.join(",");
    sql.push_str(" ");
    sql.push_str(field);
    sql.push_str(" in(");
    sql.push_str(join.as_str());
    sql.push_str(") ");
    (sql, ret_args)
}

/**
gorm named sql支持
# Examples

```
    use caisin::dbs::named_sql_expr;
    use std::collections::HashMap;

    use anyhow::{anyhow, Result};
    use rbs::{to_value, Value};
    let sql = "select * from user where id=@id and cuid=@cuid and name='你好' and hh=? and gp=? and a='?'";
    let old_args = vec![to_value!("haha"), to_value!("goupi"), to_value!("xx")];
    let mut named_args: HashMap<String, Value> = HashMap::new();
    named_args.insert("id".to_string(), to_value!(1));
    named_args.insert("cuid".to_string(), to_value!("adad1da"));
    match named_sql_expr(sql, old_args, named_args) {
        Ok((ret_sql, ret_args)) => {
            assert_eq!(&ret_sql, "select * from user where id=? and cuid=? and name='你好' and hh=? and gp=? and a='?'");
            assert_eq!(ret_args, vec![to_value!(1),to_value!("adad1da"),to_value!("haha"), to_value!("goupi")]);
        }
        Err(err) => {
            println!("fail:{err}")
        }
    }
```
*/
pub fn named_sql_expr(
    sql: &str,
    mut old_args: Vec<Value>,
    named_args: HashMap<String, Value>,
) -> Result<(String, Vec<Value>)> {
    let mut ret_args = vec![];
    let mut start_idx = 0;
    let mut idx = 0;
    let mut ret_sql = String::new();
    let mut in_cma = false;
    old_args.reverse();
    for c in sql.chars() {
        if c == '@' && start_idx == 0 {
            start_idx = idx;
            ret_sql.push('?');
        }
        if c == '\'' {
            in_cma = !in_cma;
        }
        if c == ' ' && start_idx > 0 {
            let name = &sql[start_idx + 1..idx];
            start_idx = 0;
            match named_args.get(name) {
                Some(value) => ret_args.push(value.clone()),
                None => {
                    return Err(anyhow!("key {name} not exists!"));
                }
            }
        }
        if c == '?' && !in_cma {
            match old_args.pop() {
                Some(value) => {
                    ret_args.push(value);
                }
                None => {
                    return Err(anyhow!("old param num not enough"));
                }
            }
        }
        if start_idx == 0 {
            ret_sql.push(c);
        }
        idx += 1;
    }
    Ok((ret_sql, ret_args))
}

#[test]
fn test_sql() {}
