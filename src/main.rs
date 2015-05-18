extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let mut router = Router::new();

    fn items_path(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "[]")))
    }

    router.get("/items", items_path);

    Iron::new(router).http("localhost:3000").unwrap();
    println!("On 3000");
}
