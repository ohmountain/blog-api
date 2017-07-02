extern crate mysql;

use std::sync::Arc;
use self::mysql::from_row;
use self::mysql::Pool;

use super::Type;
use super::Types;

pub fn get_types(arc: Arc<Pool>) -> Types {

    let pool = Arc::try_unwrap(arc).unwrap_err();

    let mut types = Types  { types: Vec::new() };

    for row in pool.prep_exec("SELECT id, title, sort FROM types", ()).unwrap() {
        let (id, title, sort): (u32, String, u8) = from_row(row.unwrap());

        types.types.push(Type {
            id: id,
            title: title,
            sort: sort
        })
    }

    types
}
