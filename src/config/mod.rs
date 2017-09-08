extern crate dotenv;

use self::dotenv::dotenv;
use std::env;

/// MySQL 配置结构
pub struct MySQLConf {
    pub host: Option<String>,
    pub port: Option<String>,
    pub user: Option<String>,
    pub pass: Option<String>,
    pub database: Option<String>
}

pub struct RedisConfig {
    pub host: String,
    pub port: String,
    pub pass: Option<String>,
    pub db: Option<String>
}

pub fn get_mysql_config() -> MySQLConf {
    dotenv().ok();

    let host = env::var("mysql_host").unwrap_or("localhost".into());
    let port = env::var("mysql_port").unwrap_or("3306".into());
    let user = env::var("mysql_user").unwrap_or("root".into());
    let pass = env::var("mysql_pass").unwrap_or("123456".into());
    let dbnm = env::var("mysql_database").expect("Database must be set");

    MySQLConf {
        host: Some(String::from(host)),
        port: Some(String::from(port)),
        user: Some(String::from(user)),
        pass: Some(String::from(pass)),
        database: Some(String::from(dbnm))
    }
}

pub fn get_redis_config() -> RedisConfig {
    dotenv().ok();

    let host = env::var("redis_host").unwrap_or("127.0.0.1".into());
    let port = env::var("redis_port").unwrap_or("6379".into());

    let db = (||{
        let db = env::var("redis_db");

        match db {
            Ok(data) => Some(data.into()),
            Err(_) => None
        }
    })();

    let pass = (||{
        let pass = env::var("redis_pass");

        match pass {
            Ok(data) => Some(data.into()),
            Err(_) => None
        }
    })();

    RedisConfig {
        host: host,
        port: port,
        pass: pass,
        db: db
    }
}
