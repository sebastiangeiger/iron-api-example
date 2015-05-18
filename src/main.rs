extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json::{Json, ToJson};

fn main() {
    let mut router = Router::new();

    fn items_path(_: &mut Request) -> IronResult<Response> {
        let result : Vec<String> = Vec::new();
        let json_obj : Json = result.to_json();
        let json_str : String = json_obj.to_string();
        Ok(Response::with((status::Ok, json_str)))
    }

    router.get("/items", items_path);

    Iron::new(router).http("localhost:3000").unwrap();
    println!("On 3000");
}
