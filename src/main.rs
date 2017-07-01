extern crate iron;
extern crate router;
extern crate api as blog;

use iron::prelude::*;
use router::Router;
use blog::api::read::{ get_types };

fn main() {

    let mut router = Router::new();
    router.get("/get_types", get_types, "get_types");

    Iron::new(router).http("localhost:3000").unwrap();
}
