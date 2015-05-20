extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use std::io::Read;

#[derive(RustcDecodable, RustcEncodable)]
struct Item {
    name: String,
}

#[allow(dead_code)]
fn main() {
    let mut router = Router::new();

    fn items_path(_: &mut Request) -> IronResult<Response> {
        let mut result : Vec<Item> = Vec::new();
        result.push(Item { name: "Bananas".to_string() });
        match json::encode(&result) {
            Ok(json_str) => Ok(Response::with((status::Ok, json_str))),
            Err(_) => Ok(Response::with((status::InternalServerError, "")))
        }
    }

    fn create_item_path(request: &mut Request) -> IronResult<Response> {
        let ref mut body = request.body;
        let mut body_string = String::new();
        match body.read_to_string(&mut body_string) {
            Ok(_) => Ok(Response::with((status::Ok, body_string))),
            Err(_) => Ok(Response::with((status::InternalServerError, "")))
        }
    }

    router.get("/items", items_path);
    router.post("/items", create_item_path);

    Iron::new(router).http("localhost:3000").unwrap();
    println!("On 3000");
}

#[cfg(test)]
mod tests {
    use Item;
    use rustc_serialize::json;

    #[test]
    fn test_item_serialization() {
        let item = Item {
            name: "Bananas".to_string()
        };
        assert_eq!(json::encode(&item).unwrap(), "{\"name\":\"Bananas\"}".to_string());
    }
}
