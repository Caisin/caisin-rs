mod dbs;
mod gen;
mod models;

// mod gencode;
use clap::Parser;

use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use rbs::to_value;
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
    let uids = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    //wx28="mysql://quick2_cswangmwl:c4wnd7Xwj8nSf6WA@8.136.203.80:3306/quick2_cswangmwl?ssl-mode=disabled"

    let user_type = 2;
    let rb = Rbatis::new();
    rb.init(MysqlDriver {}, "mysql://root:root@127.0.0.1:3306/test")
        .unwrap();

    let chunks = uids.chunks(3);
    for sub_uids in chunks {
        let join = sub_uids.to_vec();
        let mut ids = vec![];
        for ele in join {
            ids.push(ele.to_string());
        }
        let ids_str = ids.join(",");
        let sql = format!("update cps_member set userType=? where uid in ({ids_str})");
        match rb.exec(&sql, vec![to_value!(user_type)]).await {
            Ok(o) => {
                println!("{o}");
            }
            Err(e) => {
                println!("{e}");
            }
        };
    }
}
