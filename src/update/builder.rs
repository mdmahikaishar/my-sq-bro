use super::Update;
use std::mem;

#[derive(Default)]
pub struct UpdateBuilder {
    table_name: String,
    data: Vec<(String, String)>,
    when: Vec<(String, String)>,
}

impl UpdateBuilder {
    pub fn table_name(mut self, table_name: &str) -> Self {
        self.table_name = table_name.to_string();

        self
    }

    pub fn data(mut self, key: &str, value: &str) -> Self {
        self.data.push((key.to_string(), value.to_string()));

        self
    }

    ///
    ///
    /// when("id", "1") -> raw_when("`table`.`id`", 1);    
    pub fn when(self, key: &str, value: &str) -> Self {
        let key = format!("`{}`.`{}`", self.table_name, key);

        self.raw_when(&key, value)
    }

    pub fn raw_when(mut self, key: &str, value: &str) -> Self {
        self.when.push((key.to_string(), value.to_string()));

        self
    }

    pub fn build(mut self) -> Update {
        Update {
            table_name: mem::take(&mut self.table_name),
            data: mem::take(&mut self.data),
            when: mem::take(&mut self.when),
        }
    }
}
