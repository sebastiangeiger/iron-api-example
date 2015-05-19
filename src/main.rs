extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
struct Item {
    name: String,
}

#[allow(dead_code)]
fn main() {
    let mut router = Router::new();

    fn items_path(_: &mut Request) -> IronResult<Response> {
        let result : Vec<String> = Vec::new();
        let json_str = json::encode(&result).unwrap();
        Ok(Response::with((status::Ok, json_str)))
    }

    router.get("/items", items_path);

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
