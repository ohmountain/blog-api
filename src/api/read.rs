extern crate iron;
extern crate mysql;
extern crate redis;
extern crate params;
extern crate serde_json;
extern crate persistent;

use self::iron::prelude::*;
use self::iron::status;
use self::iron::{Headers, headers};
use self::persistent::Read;
use self::redis::Commands;

use super::super::model::Type;
use super::super::connection::{ MyPool };
use super::super::model::read::get_types as m_get_types;
use super::super::connection::get_redis_connection;

#[derive(Serialize, Deserialize)]
pub struct ReturnTypes {
    code: u16,
    types: Vec<Type>
}

pub fn get_types(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<MyPool>>().unwrap();

    let mut headers = Headers::new();
    headers.set(headers::ContentType::json());
    headers.set(headers::Server("MKD 1.0".into()));


    let types = m_get_types(pool);

    let ret   = ReturnTypes { code: 200, types: types.types };
    let data  = serde_json::to_string(&ret).unwrap_or("".into());

    headers.set(headers::ContentLength(data.len() as u64));

    let response = Response {
        headers: headers,
        status: Some(status::Ok),
        body: Some(Box::new(data)),
        extensions: iron::typemap::TypeMap::new()
    };

    Ok(response)
}

pub fn cache_type(_: &mut Request) -> IronResult<Response> {

    let mut headers = Headers::new();
    headers.set(headers::ContentType::json());
    headers.set(headers::Server("MKD 1.0".into()));

    redis_set("name".into(), "renshan".into()).unwrap();

    let data: String  = "{\"name\":\"renshan\"}".into();

    headers.set(headers::ContentLength(data.len() as u64));

    let response = Response {
        headers: headers,
        status: Some(status::Ok),
        body: Some(Box::new(data)),
        extensions: iron::typemap::TypeMap::new()
    };

    Ok(response)
}

fn redis_set(k: String, v: String) -> redis::RedisResult<()> {

    let conn: redis::Connection = get_redis_connection().unwrap();
    let _: () = try!(conn.set(k, v));

    Ok(())
}
