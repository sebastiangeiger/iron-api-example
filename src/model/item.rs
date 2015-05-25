#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq)]
pub struct Item {
    pub name: String,
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
