extern crate iron;
extern crate mysql;
extern crate params;
extern crate serde_json;
extern crate persistent;
extern crate chrono;

use std::u8;
use std::u32;
use std::str::FromStr;
use self::iron::prelude::*;
use self::iron::status;
use self::iron::{Headers, headers};
use self::persistent::Read;
use self::params::{Params, FromValue };

use super::super::model::{ Type, Post };
use super::super::connection::{ MyPool };
use super::super::model::write::{ post_type as m_post_type, post_blog as m_post_blog };

#[derive(Serialize, Deserialize)]
pub struct ReturnType {
    code: u16,
    data: Option<Type>,
    message: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct ReturnPost {
    code: u16,
    data: Option<Post>,
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

pub fn post_blog(req: &mut Request) -> IronResult<Response> {

    let mut headers = Headers::new();
    headers.set(headers::ContentType::json());
    headers.set(headers::Server("MKD 1.0".into()));

    let mut ret = ReturnPost { code: 200, data: None, message: None };

    let title   = get_param(req, "title");
    let type_id = get_param(req, "type_id");
    let body    = get_param(req, "body");

    if title.is_none() || type_id.is_none() || body.is_none() {
        ret.code = 400;
        ret.message = Some("Require params".into());

        let data = serde_json::to_string(&ret).unwrap();

        let response = Response {
            headers: headers,
            status: Some(status::Ok),
            body: Some(Box::new(data)),
            extensions: iron::typemap::TypeMap::new()
        };

        return Ok(response);
    }

    let   _title = title.unwrap();
    let _type_id = u32::from_str(type_id.unwrap().as_str()).unwrap_or(1);
    let    _body = body.unwrap();

    let now: chrono::DateTime<chrono::Local> = chrono::Local::now();

    let mut post = Post {
        id: None,
        body: _body,
        title: _title,
        type_id: _type_id,
        created_at: now.timestamp().to_string(),
        updated_at: now.timestamp().to_string()
    };

    let pool = req.get::<Read<MyPool>>().unwrap();

    m_post_blog(pool, &mut post);

    if post.id.is_none() {
        ret.code = 500;
        ret.message = Some("Insert error".into());
    } else {
        ret.data = Some(post);
    }

    let data = serde_json::to_string(&ret).unwrap();

    let response = Response {
        headers: headers,
        status: Some(status::Ok),
        body: Some(Box::new(data)),
        extensions: iron::typemap::TypeMap::new()
    };

    return Ok(response);
}

fn get_param(req: &mut Request, key: &str) -> Option<String> {
    let map = req.get_ref::<Params>().unwrap();

    let wrapper = map.get(key);

    match wrapper {
        Some(val) => {
            return Some(String::from_value(val).unwrap_or(String::new()));
        },
        None => {
            return None;
        }
    }
}
