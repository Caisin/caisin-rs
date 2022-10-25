mod dbs;
mod gen;
mod models;

use std::{sync::atomic::Ordering, thread::sleep, time::Duration};

use caisin::bars;
// mod gencode;
use clap::Parser;

use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use rbs::{to_value, Value};
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
    let _bar = bars::print_use_time("测试耗时:");
    //wx28="mysql://quick2_cswangmwl:c4wnd7Xwj8nSf6WA@8.136.203.80:3306/quick2_cswangmwl?ssl-mode=disabled"
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");

    let rb = Rbatis::new();
    rb.init(MysqlDriver {}, "mysql://root:root@127.0.0.1:3306/test")
        .unwrap();

    let list = vec![1, 2, 3, 4, 5];

    let chunks = list.chunks(2);
    let sql = "update cps_member set userType=? where";
    let args = vec![to_value!(2)];
    for sub_uids in chunks {
        let (sql, exec_args) = caisin::dbs::sql_in(sql, "uid", &args, sub_uids.to_vec());
        match rb.exec(&sql, exec_args).await {
            Ok(o) => {
                println!("{o}");
            }
            Err(e) => {
                println!("{e}");
            }
        };
    }
    sleep(Duration::from_secs(1));
}
