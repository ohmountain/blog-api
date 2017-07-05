extern crate mysql;
extern crate redis;

use std::sync::Arc;
use self::mysql::from_row;
use self::mysql::Pool;
use self::redis::Commands;

use super::Type;
use super::Types;
use super::super::connection::get_redis_connection;

pub fn get_types(arc: Arc<Pool>) -> Types {

    let pool = Arc::try_unwrap(arc).unwrap_err();

    let mut types = Types  { types: Vec::new() };

    for row in pool.prep_exec("SELECT id, title, sort FROM types", ()).unwrap() {
        let (id, title, sort): (u32, String, u8) = from_row(row.unwrap());

        types.types.push(Type {
            id: Some(id),
            title: title,
            sort: sort
        })
    }

    types
}

pub fn get_redis_key(key: &String) -> Option<String> {

    let conn: redis::Connection;

    match get_redis_connection() {
        Ok(connection) => conn = connection,
        Err(_) => return None,
    };


    let res: Option<String> = conn.get(key).unwrap_or(None);

    res
}
