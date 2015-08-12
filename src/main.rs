extern crate iron;
extern crate router;
extern crate rusqlite;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::{status, AroundMiddleware, Handler};
use router::Router;
use std::io::Read;

mod model;
use model::{Item,ItemMapper,ItemCollection};

struct Bouncer;
struct BouncerHandler<H: Handler> {
    handler: H
}

impl Bouncer {
    fn new() -> Bouncer {
        Bouncer
    }
}

impl AroundMiddleware for Bouncer {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(BouncerHandler { handler: handler }) as Box<Handler>
    }
}

impl<H: Handler> Handler for BouncerHandler<H> {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        match request.headers.get_raw("X-Auth-Token") {
            None => Ok(Response::with((status::Unauthorized, "{\"error\": \"Please supply a X-Auth-Token header\"}"))),
            Some(_) => self.handler.handle(request)
        }
    }
}
#[allow(dead_code)]
fn main() {
    let mut router = Router::new();
    ItemMapper::new().create_table();

    fn items_path(_: &mut Request) -> IronResult<Response> {
        let item_mapper = ItemMapper::new();
        let items : ItemCollection = item_mapper.all();
        match items.to_json() {
            Ok(json_str) => Ok(Response::with((status::Ok, json_str))),
            Err(_) => Ok(Response::with((status::InternalServerError, "")))
        }
    }

    fn create_item_path(request: &mut Request) -> IronResult<Response> {
        let ref mut body = request.body;
        let mut body_string = String::new();
        match body.read_to_string(&mut body_string) {
            Ok(_) => {
                match Item::from_json(&body_string) {
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

    let authorizing_router = Bouncer::new().around(Box::new(router));
    Iron::new(authorizing_router).http("localhost:3000").unwrap();
    println!("On 3000");
}
