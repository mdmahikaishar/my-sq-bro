use super::join::{Join, JoinBuilder};
use super::Select;
use std::mem;

#[derive(Default)]
pub struct SelectBuilder {
    table_name: String,
    attrs: Vec<String>,
    when: Vec<(String, String)>,
    joins: Vec<Join>,
}

impl SelectBuilder {
    pub fn table_name(mut self, table_name: &str) -> Self {
        self.table_name = table_name.to_string();

        self
    }

    pub fn attr(mut self, name: &str) -> Self {
        self.attrs.push(name.to_string());

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

    pub fn join(mut self, join: JoinBuilder) -> Self {
        let join = join.build();

        self.joins.push(join);

        self
    }

    pub fn build(mut self) -> Select {
        Select {
            table_name: mem::take(&mut self.table_name),
            attrs: mem::take(&mut self.attrs),
            when: mem::take(&mut self.when),
            joins: mem::take(&mut self.joins),
        }
    }
}
