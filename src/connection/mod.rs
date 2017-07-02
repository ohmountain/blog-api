extern crate iron;
extern crate mysql;
extern crate dotenv;

use std::env;
use std::u16;
use std::str::FromStr;
use self::dotenv::dotenv;
use self::mysql::Pool;
use self::mysql::OptsBuilder;
use self::mysql::conn::Opts;
use self::iron::typemap::Key;


pub struct MyPool;

impl Key for MyPool { type Value = Pool; }

pub fn get_connection() -> mysql::Pool {

    let opts = get_options();
    let pool = mysql::Pool::new(opts).unwrap();

    pool
}

fn  get_options() -> Opts {

    dotenv().ok();

    let host = env::var("host").unwrap_or("localhost".into());
    let port = env::var("port").unwrap_or("3306".into());
    let user = env::var("user").unwrap_or("root".into());
    let pass = env::var("pass").unwrap_or("".into());
    let dbnm = env::var("database").expect("Database must be set");

    let mut builder = OptsBuilder::new();
    builder.ip_or_hostname(Some(host))
        .tcp_port(u16::from_str(port.as_str()).unwrap_or(3306))
        .user(Some(user))
        .pass(Some(pass))
        .db_name(Some(dbnm));

    Opts::from(builder)
}
