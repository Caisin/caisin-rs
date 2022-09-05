mod db;
mod gen;
mod models;
use clap::Parser;
use gen::gen_rbatis::gen_rbatis;
#[macro_use]
extern crate rbatis;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// code generate type
    #[clap(short, long, value_parser, default_value = "rbatis")]
    gen_type: String,
    /// dburl
    #[clap(short, long, value_parser, required = true)]
    db_url: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match &args.gen_type as &str {
        "rbatis" => {
            gen_rbatis(args.db_url).await;
        }
        _ => {
            println!("unsupport gen type: [{}]", args.gen_type)
        }
    }
}
