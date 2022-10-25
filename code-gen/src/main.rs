mod dbs;
mod gen;
mod models;

// mod gencode;
use clap::Parser;

use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use rbs::Value;
#[macro_use]
extern crate rbatis;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// code generate type
    #[clap(short, long, value_parser, default_value = "rbatis")]
    gen_type: String,
    /// dburl
    #[clap(short, long, value_parser, required = true)]
    db_url: String,
    /// 输出目录
    #[clap(short, long, value_parser, default_value = "gencode")]
    out_path: String,
}

#[tokio::main]
async fn main() {
    //wx28="mysql://quick2_cswangmwl:c4wnd7Xwj8nSf6WA@8.136.203.80:3306/quick2_cswangmwl?ssl-mode=disabled"
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");

    let rb = Rbatis::new();
    rb.init(MysqlDriver {}, "mysql://root:root@127.0.0.1:3306/go-admin")
        .unwrap();

    let sql = "select * from sys_menu where";
    let args: Vec<Value> = vec![];
    let var_name = vec![1, 2, 3, 4, 5];
    for item in var_name.chunks(2) {
        let (sql, args) = caisin::dbs::sql_in(sql, "menu_id", &args, item.to_vec());
        let _ = caisin::dbs::fetch_map_list(&rb, sql.as_str(), args).await;
    }
}
