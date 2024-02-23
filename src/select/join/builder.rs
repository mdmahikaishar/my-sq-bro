use super::Join;
use std::mem;

pub enum JoinType {
    Left,
    Right,
    Inner,
    Outer,
}

impl Default for JoinType {
    fn default() -> Self {
        Self::Left
    }
}

impl ToString for JoinType {
    fn to_string(&self) -> String {
        match self {
            JoinType::Left => "LEFT JOIN".to_string(),
            JoinType::Right => "RIGHT JOIN".to_string(),
            JoinType::Inner => "INNER JOIN".to_string(),
            JoinType::Outer => "OUTER JOIN".to_string(),
        }
    }
}

#[derive(Default)]
pub struct JoinBuilder {
    join_type: JoinType,
    table_name: String,
    attrs: Vec<String>,
    on: Vec<(String, String)>,
}

impl JoinBuilder {
    pub fn join_type(mut self, join_type: JoinType) -> Self {
        self.join_type = join_type;

        self
    }

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
    /// on("student_id", 1) -> raw_on("`table`.`student_id`", 1)
    pub fn on(self, key: &str, value: &str) -> Self {
        let key = format!("`{}`.`{}`", self.table_name, key);

        self.raw_on(&key, value)
    }

    pub fn raw_on(mut self, key: &str, value: &str) -> Self {
        self.on.push((key.to_string(), value.to_string()));

        self
    }

    pub fn build(mut self) -> Join {
        Join {
            join_type: mem::take(&mut self.join_type),
            table_name: mem::take(&mut self.table_name),
            attrs: mem::take(&mut self.attrs),
            on: mem::take(&mut self.on),
        }
    }
}
