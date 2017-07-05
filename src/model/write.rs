extern crate mysql;
extern crate redis;
extern crate serde_json;

use std::sync::Arc;

use self::mysql::Pool;
use self::redis::Commands;

use super::Type;
use super::super::connection::get_redis_connection;

pub fn post_type(arc: Arc<Pool>, _type: &mut Type) -> bool {

    let pool = Arc::try_unwrap(arc).unwrap_err();

    let res  = pool.prep_exec("INSERT INTO types(title, sort) VALUES (?,?)", (&_type.title, &_type.sort));

    match res {
        Ok(t) => {
            _type.id = Some(t.last_insert_id() as u32);

            redis_rm_k("_cached_types".into()).unwrap();
            true
        }

        Err(_) => false
    }
}



pub fn redis_set_kv(k: String, v: String) -> redis::RedisResult<()> {

    let conn: redis::Connection;

    match get_redis_connection() {
        Ok(connection) => conn = connection,
        Err(_) => return Err(redis::RedisError::from((redis::ErrorKind::ResponseError, "None"))),
    };

    let _: () = try!(conn.set(k, v));

    Ok(())
}

pub fn redis_rm_k(k: String) -> redis::RedisResult<()> {

    let conn: redis::Connection;

    match get_redis_connection() {
        Ok(connection) => conn = connection,
        Err(_) => {
            return Err(redis::RedisError::from((redis::ErrorKind::ResponseError, "None")));
        }
    };

    let _: () = try!(conn.del(k));

    Ok(())
}
