extern crate iron;
extern crate router;
extern crate mysql;
extern crate persistent;
extern crate api as blog;

use iron::prelude::*;
use iron::middleware::Chain;
use router::Router;
use persistent::Read;
use blog::api::read::{ get_types };
use blog::api::write::{ post_type };
use blog::connection::{ get_connection, MyPool };


fn main() {

    let pool = get_connection();

    let mut router = Router::new();
    router.get("/api/v2/types", get_types, "get_types");
    router.post("/api/v2/type", post_type, "post_type");

    let mut chain = Chain::new(router);
    chain.link(Read::<MyPool>::both(pool));

    Iron::new(chain).http("localhost:3000").unwrap();
}
