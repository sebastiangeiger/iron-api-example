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
                        Ok(Response::with((status::Ok, json::encode(&item).unwrap())))
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

#[cfg(test)]
mod tests {
    use model::{Item, ItemMapper};
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

    #[test]
    fn test_item_mapper_create_table_can_be_called_multiple_times() {
        let mapper = ItemMapper::new();
        mapper.create_table();
        mapper.create_table();
    }

    #[test]
    fn test_writing_and_reading_one_item_from_db() {
        let mapper = ItemMapper::new();
        mapper.drop_table();
        mapper.create_table();
        let item = Item {
            name: "Bananas".to_string()
        };
        mapper.insert(&item);
        assert_eq!(mapper.all(), vec![item])
    }
}
