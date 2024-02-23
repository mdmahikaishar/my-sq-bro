use super::Delete;
use std::mem;

#[derive(Default)]
pub struct DeleteBuilder {
    table_name: String,
    when: Vec<(String, String)>,
}

impl DeleteBuilder {
    pub fn table_name(mut self, table_name: &str) -> Self {
        self.table_name = table_name.to_string();

        self
    }

    ///
    ///
    /// when("id", "1") -> raw_when("`table`.`id", 1);    
    pub fn when(self, key: &str, value: &str) -> Self {
        let key = format!("`{}`.`{}`", self.table_name, key);

        self.raw_when(&key, value)
    }

    pub fn raw_when(mut self, key: &str, value: &str) -> Self {
        self.when.push((key.to_string(), value.to_string()));

        self
    }

    pub fn build(mut self) -> Delete {
        Delete {
            table_name: mem::take(&mut self.table_name),
            when: mem::take(&mut self.when),
        }
    }
}
