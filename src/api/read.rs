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

use super::super::model::{ Type, Types };
use super::super::connection::{ MyPool };
use super::super::model::read::get_types as m_get_types;
use super::super::model::read::get_redis_key;
use super::super::model::write::redis_set_kv;

#[derive(Serialize, Deserialize)]
pub struct ReturnTypes {
   code: u16,
   types: Vec<Type>
}

pub fn get_types(req: &mut Request) -> IronResult<Response> {

    let cached = get_redis_key(&("_cached_types".into()));

    let mut types = Types { types: Vec::new() };
    let mut ret   = ReturnTypes { code: 200, types: types.types };

    let data:String;

    match cached {
        Some(json) => {

            // 性能不好
            ret = serde_json::from_str(json.as_str()).unwrap();
            data  = serde_json::to_string(&ret).unwrap_or("".into());
        },
        None => {
            let pool = req.get::<Read<MyPool>>().unwrap();
            types = m_get_types(pool);

            ret.types = types.types;
            data  = serde_json::to_string(&ret).unwrap_or("".into());

            redis_set_kv("_cached_types".into(), data.clone()).unwrap();
        }
    }

    let mut headers = Headers::new();
    headers.set(headers::ContentType::json());
    headers.set(headers::Server("MKD 1.0".into()));
    headers.set(headers::ContentLength(data.len() as u64));

    let response = Response {
        headers: headers,
        status: Some(status::Ok),
        body: Some(Box::new(data)),
        extensions: iron::typemap::TypeMap::new()
    };

    Ok(response)
}
