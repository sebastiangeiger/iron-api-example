extern crate rustc_serialize;

use rustc_serialize::json;
use rustc_serialize::json::{EncoderError,DecoderError};

#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq)]
pub struct Item {
    pub name: String,
}

impl Item {
    pub fn to_json(&self) -> Result<String, EncoderError> {
        json::encode(&self)
    }

    pub fn from_json(json : &String) -> Result<Item, DecoderError> {
        json::decode(&json)
    }
}

#[cfg(test)]
mod tests {
    use rustc_serialize::json;
    use model::item::Item;

    #[test]
    fn test_item_serialization() {
        let item = Item {
            name: "Bananas".to_string()
        };
        assert_eq!(item.to_json().unwrap(), "{\"name\":\"Bananas\"}".to_string());
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
        let json = "{\"name\":\"Bananas\"}".to_string();
        let parsed_item = Item::from_json(&json).unwrap();
        assert_eq!(parsed_item, item);
    }

    #[test]
    fn test_item_parsing_with_invalid_item() {
        let json = "{\"garbage\":\"key\"}".to_string();
        let parse_result = Item::from_json(&json);
        assert!(parse_result.is_err())
    }
}
