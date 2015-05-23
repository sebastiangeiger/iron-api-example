extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use std::io::Read;

#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq)]
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
            Ok(_) => {
                let item : Result<Item,_> = json::decode(&body_string);
                match item {
                    Ok(_) => Ok(Response::with((status::Ok, body_string))),
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

    #[test]
    fn test_item_equality_with_the_same_name() {
        let item = Item {
            name: "Bananas".to_string()
        };
        let item_2 = Item {
            name: "Bananas".to_string()
        };
        assert_eq!(item, item_2);
    }

    #[test]
    fn test_item_equality_with_a_different_name() {
        let item = Item {
            name: "Bananas".to_string()
        };
        let item_2 = Item {
            name: "Apples".to_string()
        };
        assert!(item != item_2);
    }

    #[test]
    fn test_item_parsing_with_valid_item() {
        let item = Item {
            name: "Bananas".to_string()
        };
        let json = "{\"name\":\"Bananas\"}";
        let parsed_item : Item = json::decode(&json).unwrap();
        assert_eq!(parsed_item, item);
    }

    #[test]
    fn test_item_parsing_with_invalid_item() {
        let json = "{\"garbage\":\"key\"}";
        let parse_result : Result<Item,_> = json::decode(&json);
        assert!(parse_result.is_err())
    }
}
