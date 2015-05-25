extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate rusqlite;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use std::io::Read;

mod model;
use model::{Item,ItemMapper};

#[allow(dead_code)]
fn main() {
    let mut router = Router::new();
    ItemMapper::new().create_table();

    fn items_path(_: &mut Request) -> IronResult<Response> {
        let item_mapper = ItemMapper::new();
        let items : Vec<Item> = item_mapper.all();
        match json::encode(&items) {
            Ok(json_str) => Ok(Response::with((status::Ok, json_str))),
            Err(_) => Ok(Response::with((status::InternalServerError, "")))
        }
    }

    fn create_item_path(request: &mut Request) -> IronResult<Response> {
        let ref mut body = request.body;
        let mut body_string = String::new();
        match body.read_to_string(&mut body_string) {
            Ok(_) => {
                let decoding_result : Result<Item,_> = json::decode(&body_string);
                match decoding_result {
                    Ok(item) => {
                        let item_mapper = ItemMapper::new();
                        item_mapper.insert(&item);
                        Ok(Response::with((status::Ok, item.to_json().unwrap())))
                    }
                    Err(_) => Ok(Response::with((status::UnprocessableEntity, "{\"error\":\"Invalid payload\"}")))
                }
            },
            Err(_) => Ok(Response::with((status::InternalServerError, "")))
        }
    }

    router.get("/items", items_path);
    router.post("/items", create_item_path);

    Iron::new(router).http("localhost:3000").unwrap();
    println!("On 3000");
}
