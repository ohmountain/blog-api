extern crate iron;
extern crate mysql;
extern crate redis;
extern crate dotenv;

use std::env;
use std::u16;
use std::str::FromStr;
use std::marker::Sync;
use self::dotenv::dotenv;
use self::mysql::Pool;
use self::mysql::OptsBuilder;
use self::mysql::conn::Opts;
use self::iron::typemap::Key;



/// mysql 连接池的 persistent 封装
pub struct MyPool;
impl Key for MyPool { type Value = Pool; }

pub fn get_mysql_connection() -> mysql::Pool {

    let opts = get_options();
    let pool = mysql::Pool::new(opts).unwrap();

    pool
}

/// 获取mysq配置
fn  get_options() -> Opts {

    dotenv().ok();

    let host = env::var("mysql_host").unwrap_or("localhost".into());
    let port = env::var("mysql_port").unwrap_or("3306".into());
    let user = env::var("mysql_user").unwrap_or("root".into());
    let pass = env::var("mysql_pass").unwrap_or("".into());
    let dbnm = env::var("mysql_database").expect("Database must be set");

    let mut builder = OptsBuilder::new();
    builder.ip_or_hostname(Some(host))
        .tcp_port(u16::from_str(port.as_str()).unwrap_or(3306))
        .user(Some(user))
        .pass(Some(pass))
        .db_name(Some(dbnm));

    Opts::from(builder)
}

pub fn get_redis_connection() -> redis::RedisResult<redis::Connection> {

    dotenv().ok();

    let host = env::var("redis_host").unwrap_or("127.0.0.1".into());
    let port = env::var("redis_port").unwrap_or("6379".into());

    let addr = format!("redis://{}:{}/", host, port);

    let client = try!(redis::Client::open(addr.as_str()));
    let con = try!(client.get_connection());

    Ok(con)
}
