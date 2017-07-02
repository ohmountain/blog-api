extern crate iron;
extern crate mysql;
extern crate params;
extern crate serde_json;
extern crate persistent;

use self::iron::prelude::*;
use self::iron::status;
use self::iron::{Headers, headers};
use self::persistent::Read;

use super::super::model::Type;
use super::super::connection::{ MyPool };
use super::super::model::read::get_types as m_get_types;

#[derive(Serialize, Deserialize)]
pub struct Return {
    code: u16,
    types: Vec<Type>
}

pub fn get_types(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<MyPool>>().unwrap();

    let mut headers = Headers::new();
    headers.set(headers::ContentType::json());
    headers.set(headers::Server("MKD 1.0".into()));


    let types = m_get_types(pool);
    let ret   =  Return { code: 200, types: types.types };
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
