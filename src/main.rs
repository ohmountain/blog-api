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
use blog::api::write::{ post_type, post_blog };
use blog::connection::{ get_mysql_connection, MyPool };



fn main() {

    let mysql_pool = get_mysql_connection();

    let mut router = Router::new();
    router.get("/api/v2/types", get_types, "get_types");
    router.post("/api/v2/type", post_type, "post_type");
    router.post("/api/v2/blog", post_blog, "post_blog");

    let mut chain = Chain::new(router);
    chain.link(Read::<MyPool>::both(mysql_pool));

    Iron::new(chain).http("localhost:3000").unwrap();
}

