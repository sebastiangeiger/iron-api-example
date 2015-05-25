use rusqlite::SqliteConnection;
use std::path::Path;
use ::model::Item;

pub struct ItemMapper {
    connection: SqliteConnection,
}

impl ItemMapper {
    pub fn new() -> ItemMapper {
        let path = Path::new("test.sqlite3");
        ItemMapper {
            connection: SqliteConnection::open(&path).unwrap()
        }
    }

    pub fn create_table(&self) {
        self.connection.execute("CREATE TABLE IF NOT EXISTS items (id   INTEGER PRIMARY KEY,
                                                                   name TEXT NOT NULL)", &[]).unwrap();
    }

    #[allow(dead_code)]
    pub fn drop_table(&self) {
        self.connection.execute("DROP TABLE IF EXISTS items", &[]).unwrap();
    }

    pub fn insert(&self, item: &Item) {
        self.connection.execute("INSERT INTO items (name) VALUES ($1)", &[&item.name]).unwrap();
    }

    pub fn all(&self) -> Vec<Item> {
        let mut result = Vec::new();
        let mut stmt = self.connection.prepare("SELECT name FROM items").unwrap();
        let items_iter = stmt.query_map(&[], |row| {
            Item {
                name: row.get(0),
            }
        }).unwrap();
        for item in items_iter {
            result.push(item.unwrap())
        };
        result
    }
}

#[cfg(test)]
mod tests {
    use model::{Item, ItemMapper};

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
