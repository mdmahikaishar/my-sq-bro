use builder::InsertBuilder;

mod builder;

pub struct Insert {
    table_name: String,
    keys: Vec<String>,
    values: Vec<String>,
}

impl Insert {
    pub fn new() -> InsertBuilder {
        InsertBuilder::default()
    }

    pub fn exec(&self) -> String {
        // "INSERT INTO `{table}` ({keys}) VALUES ({values})",
        let mut query = String::new();

        query.push_str(&format!("INSERT INTO `{}`", self.table_name));

        query.push(' ');
        query.push_str(&format!("({})", self.get_keys_string()));

        query.push(' ');
        query.push_str(&format!("VALUES ({})", self.get_values_string()));

        query
    }

    fn get_keys_string(&self) -> String {
        self.keys
            .iter()
            .map(|key| format!("`{}`", key))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn get_values_string(&self) -> String {
        self.values
            .iter()
            .map(|value| format!("\"{}\"", value))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_builder() {
        let insert_one = Insert::new()
            .table_name("users")
            .data("name", "SQL Bro")
            .build()
            .exec();

        assert_eq!(
            insert_one,
            ["INSERT INTO `users`", "(`name`)", "VALUES (\"SQL Bro\")"].join(" ")
        );

        let insert_two = Insert::new()
            .table_name("users")
            .data("name", "SQL Bro")
            .data("email", "sql@mail.com")
            .build()
            .exec();

        assert_eq!(
            insert_two,
            [
                "INSERT INTO `users`",
                "(`name`, `email`)",
                "VALUES (\"SQL Bro\", \"sql@mail.com\")"
            ]
            .join(" ")
        );
    }
}
