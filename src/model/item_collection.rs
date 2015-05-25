extern crate rustc_serialize;
use rustc_serialize::json;
use rustc_serialize::json::{EncoderError,DecoderError};

use model::item::Item;


#[derive(Debug)]
pub struct ItemCollection {
    items: Vec<Item>,
}

impl ItemCollection {
    fn new() -> ItemCollection {
        ItemCollection {
            items: Vec::new(),
        }
    }

    pub fn new_with_items(items: Vec<Item>) -> ItemCollection {
        ItemCollection {
            items: items,
        }
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn push(&mut self, item : Item) {
        self.items.push(item);
    }

    pub fn to_json(&self) -> Result<String, EncoderError> {
        rustc_serialize::json::encode(&self.items)
    }

    fn from_json(json: &String) -> Result<ItemCollection, DecoderError> {
        let items : Vec<Item> = try!(rustc_serialize::json::decode(&json));
        Ok(ItemCollection::new_with_items(items))
    }
}

impl PartialEq for ItemCollection {
    fn eq(&self, other: &ItemCollection) -> bool {
        self.items == other.items
    }
}


#[cfg(test)]
mod tests {
    use model::item::Item;
    use model::item_collection::ItemCollection;

    #[test]
    fn pushing_an_item_to_a_collection_increases_length_by_1() {
        let mut collection = ItemCollection::new();
        assert_eq!(collection.len(), 0);
        let item = Item {
            name: "Bananas".to_string()
        };
        collection.push(item);
        assert_eq!(collection.len(), 1);
    }

    #[test]
    fn json_serialization_with_no_elements() {
        let collection = ItemCollection::new();
        assert_eq!(collection.to_json().unwrap(), "[]".to_string());
    }

    #[test]
    fn json_serialization_with_one_element() {
        let mut collection = ItemCollection::new();
        let item = Item {
            name: "Oranges".to_string()
        };
        collection.push(item);
        assert_eq!(collection.to_json().unwrap(), "[{\"name\":\"Oranges\"}]".to_string());
    }

    #[test]
    fn json_parsing_with_valid_item() {
        let mut collection = ItemCollection::new();
        let item = Item {
            name: "Carrots".to_string()
        };
        collection.push(item);

        let json = "[{\"name\":\"Carrots\"}]".to_string();
        let parsed_collection = ItemCollection::from_json(&json).unwrap();
        assert_eq!(parsed_collection, collection);
    }

    #[test]
    fn json_parsing_with_invalid_item() {
        let json = "[{\"some\":\"garbage\"}]".to_string();
        let result = ItemCollection::from_json(&json);
        assert!(result.is_err());
    }

    #[test]
    fn equality_with_the_equal_items() {
        let mut collection = ItemCollection::new();
        let item = Item {
            name: "Bananas".to_string()
        };
        collection.push(item);

        let mut collection_2 = ItemCollection::new();
        let item_2 = Item {
            name: "Bananas".to_string()
        };
        collection_2.push(item_2);

        assert_eq!(collection, collection_2);
    }

    #[test]
    fn equality_with_different_number_of_items() {
        let collection = ItemCollection::new();

        let mut collection_2 = ItemCollection::new();
        let item_2 = Item {
            name: "Bananas".to_string()
        };
        collection_2.push(item_2);

        assert!(collection != collection_2);
    }
}
