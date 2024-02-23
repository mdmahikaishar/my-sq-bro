use std::mem;
use super::Insert;

#[derive(Default)]
pub struct InsertBuilder {
    table_name: String,
    keys: Vec<String>,
    values: Vec<String>,
}

impl InsertBuilder {
    pub fn table_name(mut self, table_name: &str) -> Self {
        self.table_name = table_name.to_string();

        self
    }

    pub fn data(mut self, key: &str, value: &str) -> Self {
        self.keys.push(key.to_string());
        self.values.push(value.to_string());

        self
    }

    pub fn build(mut self) -> Insert {
        // if self.table_name.is_empty() || self.keys.is_empty() || self.values.is_empty() {
        //     return None;
        // }

        Insert {
            table_name: mem::take(&mut self.table_name),
            keys: mem::take(&mut self.keys),
            values: mem::take(&mut self.values),
        }
    }
}
