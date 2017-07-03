extern crate mysql;

use std::sync::Arc;
use self::mysql::Pool;

use super::Type;

pub fn post_type(arc: Arc<Pool>, _type: &mut Type) -> bool {

    let pool = Arc::try_unwrap(arc).unwrap_err();

    let res  = pool.prep_exec("INSERT INTO types(title, sort) VALUES (?,?)", (&_type.title, &_type.sort));

    match res {
        Ok(t) => {
            _type.id = Some(t.last_insert_id() as u32);

            true
        }

        Err(_) => false
    }
}
