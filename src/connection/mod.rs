extern crate iron;
extern crate mysql;
extern crate redis;
extern crate dotenv;

use std::u16;
use std::str::FromStr;
use self::mysql::Pool;
use self::mysql::OptsBuilder;
use self::mysql::conn::Opts;
use self::iron::typemap::Key;
use config::{get_mysql_config, get_redis_config};

/// mysql 连接池的 persistent 封装
pub struct MyPool;
impl Key for MyPool { type Value = Pool; }

pub fn get_mysql_connection() -> mysql::Pool {

    let opts = get_mysql_config();

    let mut builder = OptsBuilder::new();
    builder.ip_or_hostname(opts.host)
        .tcp_port(u16::from_str(opts.port.unwrap_or(String::from("3306")).as_str()).unwrap_or(3306))
        .user(opts.user)
        .pass(opts.pass)
        .db_name(opts.database);

    let mysql_options = Opts::from(builder);

    let pool = mysql::Pool::new(mysql_options).unwrap();

    pool
}

pub fn get_redis_connection() -> redis::RedisResult<redis::Connection> {
    let config = get_redis_config();

    let host = config.host;
    let port = config.port;
    let pass = config.pass;
    let db   = config.db;

    let mut addr: String;

    if pass.is_some() {
        addr = format!("redis://{}@{}:{}/", pass.unwrap(), host, port);
    } else {
        addr = format!("redis://{}:{}/", host, port);
    }

    if db.is_some() {
        addr = format!("{}/{}", addr, db.unwrap());
    }


    let client = try!(redis::Client::open(addr.as_str()));
    let con = try!(client.get_connection());

    Ok(con)
}
