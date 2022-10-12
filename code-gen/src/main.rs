mod gen;
mod models;
// mod gencode;
use clap::Parser;
use gen::gen_rbatis::gen_rbatis;
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
    let args = Args::parse();

    match &args.gen_type as &str {
        "rbatis" => {
            gen_rbatis(&args).await;
        }
        _ => {
            println!("unsupport gen type: [{}]", args.gen_type)
        }
    }
}

#[test]
fn hh() {
    let url = "mysql://root:123456@localhost:3306/test";
    let find = url.find("://").unwrap();
    let db_type = &url[..find];
    println!("{}", db_type);
}
