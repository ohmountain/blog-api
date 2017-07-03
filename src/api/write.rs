extern crate iron;
extern crate mysql;
extern crate params;
extern crate serde_json;
extern crate persistent;

use std::u8;
use std::str::FromStr;
use self::iron::prelude::*;
use self::iron::status;
use self::iron::{Headers, headers};
use self::persistent::Read;
use self::params::{Params, FromValue };

use super::super::model::Type;
use super::super::connection::{ MyPool };
use super::super::model::write::post_type as m_post_type;

#[derive(Serialize, Deserialize)]
pub struct ReturnType {
    code: u16,
    data: Option<Type>,
    message: Option<String>
}

pub fn post_type(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<MyPool>>().unwrap();

    let mut headers = Headers::new();
    headers.set(headers::ContentType::json());
    headers.set(headers::Server("MKD 1.0".into()));

    let map = req.get_ref::<Params>().unwrap();

    let title = map.get("title");
    let sort  = map.get("sort");

    let mut ret = ReturnType { code: 200, data: None, message: None };

    match title {
        Some(t) => {

            let mut _sort = 0;

            if sort.is_some() {
                let tmp = String::from_value(sort.unwrap()).unwrap_or("0".into());
                _sort = u8::from_str(tmp.as_str()).unwrap_or(0);

            }

            let _title = String::from_value(t).unwrap();

            let mut _type = Type {
                id: None,
                title: _title,
                sort: _sort
            };

            let success: bool = m_post_type(pool, &mut _type);


            match success {
                false => ret.message = Some("Error: insert fail".into()),
                true => ret.data = Some(_type)
            }

        },
        _ => {
            ret.code = 400;
            ret.message = Some("require paramaters".into());
        }
    }

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
