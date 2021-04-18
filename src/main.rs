extern crate iron;
extern crate urlencoded;
extern crate router;

#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;
use router::Router;

fn get_form(_request : &mut Request) -> IronResult<Response>{
    return Ok(Response::new)
}


fn post_add(request : & mut Request) -> IronResult<Response>{
    return Ok(Response::new)
}

fn main() {

    let mut router = Router::new();
    router.get("/",get_form, "root");
    router.post("/add", post_add, "add");


    println!("Serving on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}
