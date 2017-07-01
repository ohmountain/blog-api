extern crate iron;
extern crate params;
extern crate serde_json;

use self::iron::prelude::*;
use self::iron::status;
use self::iron::{Headers, headers};
use self::params::{Params, Value};

use super::super::model::read::Type;
use super::super::model::read::get_types as m_get_types;

#[derive(Serialize, Deserialize)]
pub struct Return {
    code: u16,
    types: Vec<Type>
}

pub fn get_types(req: &mut Request) -> IronResult<Response> {
    let map = req.get_ref::<Params>().unwrap();


    let mut headers = Headers::new();
    headers.set(headers::ContentType::json());
    headers.set(headers::Server("MKD 1.0".into()));

    match map.find(&["key"]) {
        Some(&Value::String(ref key)) if key == "renshan" => {
            match m_get_types() {
                Some(hash) => {

                    let ret  =  Return { code: 200, types: hash.types };

                    let data = serde_json::to_string(&ret).unwrap_or("".into());

                    headers.set(headers::ContentLength(data.len() as u64));


                    let response = Response {
                        headers: headers,
                        status: Some(status::Ok),
                        body: Some(Box::new(data)),
                        extensions: iron::typemap::TypeMap::new()
                    };

                    Ok(response)
                }

                _ => {
                    let ret = Return { code: 404, types: Vec::new() };


                    let data = serde_json::to_string(&ret).unwrap_or("".into());

                    headers.set(headers::ContentLength(data.len() as u64));


                    let response = Response {
                        headers: headers,
                        status: Some(status::Ok),
                        body: Some(Box::new(data)),
                        extensions: iron::typemap::TypeMap::new()
                    };

                    Ok(response)
                }
            }
        },
        _ => Ok(Response::with(status::NotFound)),
    }
}
